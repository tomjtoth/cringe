mod crud_query;
pub mod details;
mod gps;
mod image;
mod prompt;
#[cfg(feature = "server")]
pub mod server;
pub mod websocket;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::person::{Decision, Person};
#[cfg(feature = "server")]
use crate::state::server::get_ctx;

pub static ME: GlobalSignal<Me> = Signal::global(Me::default);

/// Uses `$1` and needs the `session_id` as the 1st bound param
pub const AUTH_CTE: &str = "\
auth AS (
    UPDATE auth_sessions SET
        expires_at = NOW() + INTERVAL '30 days'
    WHERE id = $1
    AND expires_at > NOW()
    AND csrf_token IS NULL
    RETURNING email
)";

#[get("/api/me")]
async fn get_me() -> Result<Me> {
    let mut me = Me::default();

    if let (Some(sess_id), pool) = get_ctx().await {
        use sqlx::types::Json;

        Json(me) = sqlx::query_scalar::<_, Json<Me>>(&format!(
            r#"
            WITH {AUTH_CTE},

            profile AS (
                SELECT
                    id,
                    name,
                    gender,
                    born,
                    height,
                    education,
                    occupation,
                    location,
                    hometown,

                    seeking,
                    relationship_type,

                    jsonb_build_object(
                        'has',      kids_has,
                        'wants',    kids_wants
                    ) AS kids,

                    habits_drinking,
                    habits_smoking,
                    habits_marijuana,
                    habits_drugs,

                    (
                        SELECT coalesce(
                            jsonb_agg(to_jsonb(up) ORDER BY up.position),
                            '[]'::jsonb
                        )
                        FROM user_prompts up
                        WHERE up.user_id = u.id
                    ) AS prompts,

                    (
                        SELECT coalesce(
                            jsonb_agg(to_jsonb(ui) ORDER BY ui.position),
                            '[]'::jsonb
                        )
                        FROM user_images ui
                        WHERE ui.user_id = u.id
                    ) AS images

                FROM auth a
                JOIN users u ON a.email = u.email
            )

            SELECT jsonb_build_object(
                'authenticated', (SELECT count(*) > 0 FROM auth),
                'profile', (SELECT row_to_json(p) FROM profile AS p)
            )
            "#
        ))
        .bind(&sess_id)
        .fetch_one(&pool)
        .await?;
    }

    Ok(me)
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Me {
    pub authenticated: bool,
    pub profile: Option<Person>,
}

#[post("/api/decide")]
pub async fn decide(target_id: i32, decision: Decision) -> Result<bool> {
    let mut res = false;

    if let (Some(sess_id), pool) = get_ctx().await {
        let db_res = sqlx::query(&format!(
            "
            WITH {AUTH_CTE}

            INSERT INTO user_decisions (actor_user_id, target_user_id, decision)
            SELECT u.id, $2, $3
            FROM auth a
            INNER JOIN users u ON a.email = u.email
            ON CONFLICT (actor_user_id, target_user_id) DO UPDATE
            SET decision = EXCLUDED.decision
            "
        ))
        .bind(&sess_id)
        .bind(target_id)
        .bind(decision)
        .execute(&pool)
        .await?;

        res = db_res.rows_affected() > 0;
    }

    Ok(res)
}

pub fn init_client() -> Result<(), RenderError> {
    let initial_state = use_server_future(get_me)?;

    if let Some(Ok(me)) = initial_state() {
        *ME.write() = me;
    } else {
        #[cfg(feature = "server")]
        error!("GET /api/me returned: {:?}", initial_state.value());
    }

    // use_gps_watch();

    Ok(())
}
