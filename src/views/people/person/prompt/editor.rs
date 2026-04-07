use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::person::Prompt;
use crate::state::client::{AUTH_CTE, ME};
use crate::views::people::person::utils::class_canceler_deleter;
use crate::views::people::person::{container::Container, ResourceCtx, Sorter};

#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Default, Debug)]
struct Response {
    authorized: bool,
    sorted: i64,
    deleted: i64,
    updated: i64,
    inserted_id: Option<i32>,
}

#[put("/api/me/prompts")]
async fn mod_prompt(prompt: Prompt, sorter: Sorter) -> Result<Response> {
    use sqlx::types::Json;

    info!("Received {prompt:?} {sorter:?}");

    let mut res = Response::default();

    if let (Some(sess_id), pool) = crate::state::server::get_ctx().await {
        let (sorter_ids, sorter_pos): (Vec<Option<i32>>, Vec<Option<i16>>) =
            sorter.into_iter().unzip();

        Json(res) = sqlx::query_scalar::<_, Json<Response>>(&format!(
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

            arg_sorter AS (
                SELECT * FROM unnest($3::int[], $4::smallint[]) 
                AS _x(id, position)
            ),

            sorted AS (
                UPDATE user_prompts AS up
                SET position = pos.position
                FROM arg_sorter pos
                CROSS JOIN me
                WHERE up.user_id = me.id
                AND up.id = pos.id
                RETURNING up.id
            ),

            deleted AS (
                DELETE FROM user_prompts AS up
                USING arg_prompt AS p
                CROSS JOIN me
                WHERE up.user_id = me.id 
                AND up.id = p.id AND (p.title = '' OR p.body = '' OR p.position IS NULL)
                RETURNING up.id
            ),

            inserted AS (
                INSERT INTO user_prompts AS up
                    (user_id, position, title, body)
                SELECT
                    me.id, p.position, p.title, p.body
                FROM arg_prompt p
                CROSS JOIN me
                WHERE p.id IS NULL
                RETURNING new.id
            ),

            updated AS (
                UPDATE user_prompts AS up
                SET 
                    position = p.position,
                    title = p.title,
                    body = p.body
                FROM arg_prompt AS p
                CROSS JOIN me
                WHERE up.user_id = me.id AND up.id = p.id
                AND NOT (p.title = '' OR p.body = '' OR p.position IS NULL)
                RETURNING up.id
            )

            SELECT jsonb_build_object(
                'authorized', (SELECT count(*) > 0 FROM me),
                'sorted', (SELECT count(*) FROM sorted),
                'deleted', (SELECT count(*) FROM deleted),
                'updated', (SELECT count(*) FROM updated),
                'inserted_id', (SELECT id FROM inserted)
            )
            "
        ))
        .bind(&sess_id)
        .bind(Json(&prompt))
        .bind(&sorter_ids)
        .bind(&sorter_pos)
        .fetch_one(&pool)
        .await?;
    }

    Ok(res)
}

#[component]
pub(super) fn PromptEditor(src: Option<Prompt>) -> Element {
    let rcx = use_context::<ResourceCtx>();

    let max = ME
        .read()
        .profile
        .as_ref()
        .map(|p| p.prompts().len() as i16)
        .unwrap_or(0)
        + {
            // only a new prompt can be added as additional
            if src.is_none() {
                1
            } else {
                0
            }
        };

    let mut sig = use_signal(|| {
        src.unwrap_or(Prompt {
            position: Some(max - 1),
            ..Default::default()
        })
    });

    let new_but_empty =
        sig.with(|p| p.id == None && (p.title == "" || p.body == "" || p.position == None));

    let onsubmit = use_callback({
        let rcx = rcx.clone();

        move |_: Event<FormData>| {
            spawn({
                let mut rcx = rcx.clone();

                async move {
                    rcx.next_state();

                    if new_but_empty {
                        return rcx.next_state();
                    }

                    info!("Submitting prompt: {:?}", sig.read());

                    // getting a detached clone of the whole array to accommodate changes
                    let mut draft = ME
                        .read()
                        .profile
                        .as_ref()
                        .map(|p| p.prompts().clone())
                        .unwrap_or(vec![]);

                    let prompt_id = sig.read().id;
                    draft.retain(|pp| pp.id != prompt_id);

                    // achieving DELETE by not re-inserting at desired position
                    if let Some(pos) =
                        sig.with(|p| p.position.filter(|_| p.title != "" && p.body != ""))
                    {
                        draft.insert(pos as usize, sig());
                    }

                    let mut positions = vec![];

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

                    if let Ok(Response {
                        authorized: true,
                        inserted_id,
                        ..
                    }) = mod_prompt(sig(), positions).await.map(|res| {
                        info!("PUT /api/me/prompts returned: {:?}", res);
                        res
                    }) {
                        // paste the ID received from the server to its prompt based on pos
                        if let Some(pos) = sig.read().position.filter(|_| inserted_id.is_some()) {
                            draft.get_mut(pos as usize).map(|p| {
                                p.id = inserted_id;
                            });
                        }

                        // sync state
                        ME.with_mut(|p| {
                            p.profile.as_mut().map(|p| {
                                p.prompts.truncate(0);
                                p.prompts.append(&mut draft);
                            });
                        });
                    }

                    rcx.next_state();
                }
            });
        }
    });

    let disabled = !rcx.editing();

    let to_be_deleted =
        sig.with(|p| p.id.is_some() && (p.title == "" || p.body == "" || p.position == None));

    let (class, canceler, deleter) = class_canceler_deleter(new_but_empty, to_be_deleted);

    rsx! {
        Container { class: "{class} pt-10 pb-20", onsubmit,

            input {
                class: "min-w-20 w-full",
                placeholder: "Title",
                disabled,
                value: sig.read().title.as_ref(),
                oninput: move |evt| sig.write().title = evt.value(),
            }

            input {
                placeholder: "#",
                disabled,
                min: 1,
                max,
                r#type: "number",
                class: "max-w-20",
                value: sig.read().position.map(|pos| pos + 1),
                oninput: move |evt| {
                    sig.write().position = evt
                        .value()
                        .parse::<i16>()
                        .ok()
                        .filter(|&pos| 1 <= pos && pos <= max)
                        .map(|pos| pos - 1);
                },
            }

            textarea {
                class: "col-span-2 text-2xl",
                placeholder: "Body",
                disabled,
                value: sig.read().body.as_ref(),
                oninput: move |evt| sig.write().body = evt.value(),
            }

            // buttons
            {canceler}
            {deleter}
        }
    }
}
