use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    models::person::{Decision, Person},
    state::client::gps::use_gps_watch,
};

#[cfg(feature = "server")]
use crate::state::server::get_ctx;

mod gps;

/// **outer** option indicates an **authorized session**
///
/// **inner** option holds the **user profile** if exists
pub static ME: GlobalSignal<Option<Option<Person>>> = Signal::global(|| None);

#[get("/api/me")]
async fn get_me() -> Result<AuthResponse> {
    if let (Some(sess_id), pool) = get_ctx().await {
        let (authenticated, profile) =
            sqlx::query_as::<_, (bool, Option<sqlx::types::Json<Person>>)>(
                r#"
                    WITH auth AS (
                        SELECT email FROM auth_sessions 
                        WHERE id = $1 AND expires_at > NOW() AND csrf_token IS NULL
                    ),
                    profile AS (
                        SELECT
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

                            json_build_object(
                                'has',      kids_has,
                                'wants',    kids_wants
                            ) AS kids,

                            json_build_object(
                                'drinking',     habits_drinking,
                                'smoking',      habits_smoking,
                                'marijuana',    habits_marijuana,
                                'drugs',        habits_drugs
                            ) AS habits,

                            (
                                SELECT coalesce(
                                    json_agg(row_to_json(pp) ORDER BY pp.position),
                                    '[]'
                                )
                                FROM user_prompts pp
                                WHERE pp.user_id = u.id
                            ) as prompts,

                            (
                                SELECT coalesce(
                                    json_agg(row_to_json(up) ORDER BY up.position),
                                    '[]'
                                )
                                FROM user_pictures up
                                WHERE up.user_id = u.id
                            ) AS pictures

                        FROM auth a
                        JOIN users u ON a.email = u.email
                    )

                    SELECT 
                        (SELECT count(*) > 0 FROM auth),
                        (SELECT row_to_json(profile) FROM profile)
                    "#,
            )
            .bind(&sess_id)
            .fetch_one(&pool)
            .await?;

        return Ok(AuthResponse(authenticated, profile.map(|s| s.0)));
    }

    Ok(AuthResponse(false, None))
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse(pub bool, pub Option<Person>);

#[post("/api/decide")]
pub async fn decide(target_id: i32, decision: Decision) -> Result<bool> {
    if let (Some(sess_id), pool) = get_ctx().await {
        let val = sqlx::query(
            "
            INSERT INTO user_decisions (actor_user_id, target_user_id, decision)
            SELECT u.id, $2, $3
            FROM auth_sessions a
            INNER JOIN users u ON a.email = u.email
            WHERE a.id = $1 AND csrf_token IS NULL AND expires_at > NOW()
            ON CONFLICT (actor_user_id, target_user_id) DO UPDATE
            SET decision = EXCLUDED.decision
            ",
        )
        .bind(&sess_id)
        .bind(target_id)
        .bind(decision)
        .execute(&pool)
        .await?;

        return Ok(val.rows_affected() > 0);
    }

    Ok(false)
}

pub fn init_client_state() -> Result<(), RenderError> {
    let initial_state = use_server_future(get_me)?;

    if let Some(Ok(AuthResponse(authorized, profile))) = initial_state.read().as_ref() {
        if authorized.to_owned() {
            *ME.write() = Some(profile.to_owned())
        }
    }

    // use_gps_watch();

    Ok(())
}
