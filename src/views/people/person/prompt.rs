use dioxus::prelude::*;

use crate::models::person::PersonPrompt;
use crate::views::people::person::container::Container;
#[put("/api/me/prompts")]
async fn update_prompts(prompts: Vec<PersonPrompt>) -> Result<Vec<PersonPrompt>> {
    info!("Received {:?}", &prompts);

    let mut res = vec![];

    if let (Some(sess_id), pool) = crate::state::server::get_ctx().await {
        let vec_as_json = serde_json::to_value(&prompts)?;

        let mut tx = pool.begin().await?;

        res = sqlx::query_as::<_, PersonPrompt>(
            r"
            WITH me AS (
                SELECT u.id
                FROM auth_sessions a
                JOIN users u ON a.email = u.email
                WHERE a.id = $1
                AND a.csrf_token IS NULL
                AND a.expires_at > NOW()
            ),

            deleted AS (
                DELETE FROM user_prompts up
                USING me
                WHERE up.user_id = me.id
                RETURNING *
            ),

            input AS (
                SELECT
                    (p->>'id')::int AS id,
                    (p->>'position')::int AS position,
                    p->>'title' AS title,
                    p->>'body' AS body
                FROM jsonb_array_elements($2::jsonb) AS p
                WHERE EXISTS (
                    SELECT coalesce(
                        -- I need to make the deleted CTE part of the final result
                        -- otherwise no rows get deleted
                        1,

                        -- on the other hand this EXISTS clause returns nothing 
                        -- when a user had 0 prompts == 0 deleted rows == returning NULL
                        (select 1)
                    ) 
                    FROM deleted d
                )
            ),

            updated AS (
                INSERT INTO user_prompts AS up (id, user_id, position, title, body)
                SELECT
                    COALESCE(i.id, nextval('user_prompts_id_seq')),
                    me.id,
                    i.position,
                    i.title,
                    i.body
                FROM input i
                CROSS JOIN me

                ON CONFLICT (id) DO UPDATE
                SET
                    position = EXCLUDED.position,
                    title = EXCLUDED.title,
                    body = EXCLUDED.body
                WHERE up.user_id = EXCLUDED.user_id

                RETURNING *
            )

            SELECT * FROM updated
            ORDER BY position
            ",
        )
        .bind(&sess_id)
        .bind(&vec_as_json)
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;

        if res.len() != prompts.len() {
            error!("Tried updating prompts {:?}", vec_as_json);
        }
    }

    Ok(res)
}

#[component]

pub fn Prompt(prompt: Option<PersonPrompt>) -> Element {
    rsx! {
        if let Some(prompt) = prompt {
            Container {
                h3 { class: "p-2 pt-10", "{prompt.title}" }
                p { class: "p-2 pb-20 text-2xl", "{prompt.body}" }
            }
        }
    }
}
