use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::person::{Person, Prompt};
use crate::state::client::AUTH_CTE;
use crate::views::people::person::PersonCtx;
use crate::views::people::person::{container::Container, ResourceCtx};

#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Default, Debug)]
struct PromptUpdater {
    authorized: bool,
    updated_positions: bool,
    deleted_prompt: bool,
    updated_prompt: bool,
    inserted_prompt_id: Option<i32>,
}

#[put("/api/me/prompts")]
async fn mod_prompt(
    prompt: Prompt,
    positions: Vec<(Option<i32>, Option<i16>)>,
) -> Result<PromptUpdater> {
    use sqlx::types::Json;

    info!("Received {prompt:?} {positions:?}");

    let mut res = PromptUpdater::default();

    if let (Some(sess_id), pool) = crate::state::server::get_ctx().await {
        let (ids, positions): (Vec<Option<i32>>, Vec<Option<i16>>) = positions.into_iter().unzip();

        Json(res) = sqlx::query_scalar::<_, Json<PromptUpdater>>(&format!(
            r"
            WITH {AUTH_CTE},
            
            me AS (
                SELECT u.id FROM auth a
                JOIN users u ON a.email = u.email
            ),

            arg_prompt AS (
                SELECT * FROM jsonb_to_record($2) as _x(
                    id int,
                    user_id int,
                    position smallint,
                    title text,
                    body text
                )
            ),

            arg_positions AS (
                SELECT * FROM unnest($3::int[], $4::smallint[]) 
                AS _x(id, position)
            ),

            updated_pos AS (
                UPDATE user_prompts AS up
                SET position = pos.position
                FROM arg_positions pos
                CROSS JOIN me
                WHERE up.user_id = me.id
                AND up.id = pos.id
                RETURNING up.id
            ),

            deleted_prompt AS (
                DELETE FROM user_prompts AS up
                USING arg_prompt AS p
                CROSS JOIN me
                WHERE up.user_id = me.id 
                AND up.id = p.id AND (p.title = '' OR p.body = '')
                RETURNING up.id
            ),

            inserted_prompt AS (
                INSERT INTO user_prompts AS up
                    (user_id, position, title, body)
                SELECT
                    me.id, p.position, p.title, p.body
                FROM arg_prompt p 
                CROSS JOIN me
                WHERE p.id IS NULL
                RETURNING new.id
            ),

            updated_prompt AS (
                UPDATE user_prompts AS up
                SET 
                    position = p.position,
                    title = p.title,
                    body = p.body
                FROM arg_prompt AS p
                CROSS JOIN me
                WHERE up.user_id = me.id AND up.id = p.id
                AND NOT (p.title = '' OR p.body = '')
                RETURNING up.id
            )

            SELECT jsonb_build_object(
                'authorized', (SELECT count(*) > 0 FROM me),
                'updated_positions', (SELECT count(*) > 0 FROM updated_pos),
                'deleted_prompt', (SELECT count(*) > 0 FROM deleted_prompt),
                'updated_prompt', (SELECT count(*) > 0 FROM updated_prompt),
                'inserted_prompt_id', (SELECT id FROM inserted_prompt LIMIT 1)
            )
            "
        ))
        .bind(&sess_id)
        .bind(Json(&prompt))
        .bind(&ids)
        .bind(&positions)
        .fetch_one(&pool)
        .await?;
    }

    Ok(res)
}

#[component]
pub(super) fn PromptEditor(src: Option<Prompt>) -> Element {
    let pcx = use_context::<PersonCtx>();
    let rcx = use_context::<ResourceCtx>();

    let mut sig = use_signal(|| src.unwrap_or(Prompt::default()));

    let max = (pcx.person)().prompts().len() + {
        // only a new prompt can be added as additional
        if sig().id.is_none() {
            1
        } else {
            0
        }
    };

    // this is the submit handler I failed to attach to the Container's would-be form tag
    use_resource({
        let rcx = rcx.clone();
        let pcx = pcx.clone();

        move || {
            let mut rcx = rcx.clone();
            let pcx = pcx.clone();

            async move {
                if rcx.submitting() {
                    info!("Submitting prompt: {:?}", sig());

                    let Some(prompt_pos) = sig().position else {
                        // position must be defined
                        return;
                    };

                    // getting a detached clone of the whole array to accommodate changes
                    let mut draft = (pcx.person)().prompts().clone();
                    let mut positions = vec![];

                    let prompt_id = sig().id;
                    draft.retain(|pp| pp.id != prompt_id);

                    // DELETE by not re-inserting at desired position
                    if !(sig().title == "" || sig().body == "") {
                        draft.insert(prompt_pos as usize, sig());
                    }

                    // adjust positions of each prompt
                    for (idx, prompt) in draft.iter_mut().enumerate() {
                        prompt.position.as_mut().map(|pos| {
                            let idx = idx as i16;

                            if *pos != idx {
                                *pos = idx;
                                positions.push((prompt.id, Some(idx)));
                            }
                        });
                    }

                    if let Ok(res) = mod_prompt(sig(), positions).await {
                        info!("PUT /api/me/prompts returned: {:?}", res);

                        if let PromptUpdater {
                            authorized: true,
                            inserted_prompt_id,
                            ..
                        } = res
                        {
                            if inserted_prompt_id.is_some() {
                                // paste the ID received from the server to its prompt based on pos
                                draft.get_mut(prompt_pos as usize).map(|p| {
                                    p.id = inserted_prompt_id;
                                });
                            }

                            // move resource context out of the submitting phase
                            // else I get an endless loop :)
                            rcx.next_state();

                            // sync state
                            let mut msp = pcx.person;
                            msp.with_mut(|p| {
                                let Person { prompts: state, .. } = p;
                                state.truncate(0);
                                state.append(&mut draft);
                            });

                            return;
                        }
                    }
                    rcx.next_state();
                }
            }
        }
    });

    let disabled = !rcx.editing();

    rsx! {
        Container { class: "pt-10 pb-20 px-2 grid grid-cols-[1fr-auto] gap-2",
            input {
                class: "p-2 m-2 mr-0 text-xl min-w-30",
                placeholder: "Title",
                disabled,
                value: sig().title,
                onchange: move |evt| sig.with_mut(|p| p.title = evt.value()),
            }

            input {
                placeholder: "#",
                disabled,
                min: 1,
                max,
                r#type: "number",
                value: sig().position.map(|p| p + 1),
                onchange: move |evt| {
                    if let Ok(pos) = evt.value().parse::<i16>() {
                        if 0 <= pos && pos <= (max as i16) {
                            sig.with_mut(|p| p.position = Some(pos - 1));
                        }
                    }
                },
            }

            textarea {
                class: "border rounded p-2 col-span-2 text-2xl",
                placeholder: "Body",
                disabled,
                value: sig().body,
                onchange: move |evt| sig.with_mut(|p| p.body = evt.value()),
            }
        }
    }
}
