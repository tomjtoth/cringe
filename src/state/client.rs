use std::collections::HashMap;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::person::{Decision, Person};

#[cfg(feature = "server")]
use crate::state::server::get_ctx;

/// **outer** option indicates an **authorized session**
///
/// **inner** option holds the **user profile** if exists
pub static ME: GlobalSignal<Option<Option<Person>>> = Signal::global(|| None);

/// keeps track of unlogged users' decisions
pub static DECISIONS: GlobalSignal<HashMap<i32, Decision>> = Signal::global(|| HashMap::new());

#[get("/api/decisions")]
pub async fn get_decisions() -> Result<Vec<(i32, Decision)>> {
    if let (Some(session_id), pool) = get_ctx().await {
        let decisions = sqlx::query_as::<_, (i32, Decision)>(
            "
                SELECT target_user_id, decision
                FROM auth_sessions a
                JOIN users u ON u.email = a.email
                JOIN user_decisions d ON u.id = d.actor_user_id
                WHERE a.id = $1 AND csrf_token IS NULL AND a.expires_at > NOW()
                ",
        )
        .bind(&session_id)
        .fetch_all(&pool)
        .await?;

        return Ok(decisions);
    };

    Ok(vec![])
}

#[get("/api/me")]
pub async fn get_me() -> Result<AuthResponse> {
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

#[post("/api/gps")]
async fn post_gps(coords: Coords) -> Result<()> {
    if let (Some(sess_id), pool) = get_ctx().await {
        let res = sqlx::query(
            "
            UPDATE users u 
            SET gps_lon = $1, gps_lat = $2
            FROM auth_sessions a
            WHERE a.id = $3 
            AND u.email = a.email
            AND expires_at > NOW()
            ",
        )
        .bind(&coords.lon)
        .bind(&coords.lat)
        .bind(&sess_id)
        .execute(&pool)
        .await?;

        if res.rows_affected() == 0 {
            eprintln!("expired session \"{sess_id}\", nothing to update")
        }
    }

    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
struct Coords {
    lat: f64,
    lon: f64,
}

pub fn update_gps_pos() {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::{closure::Closure, JsCast};
        use web_sys::{window, GeolocationPosition, GeolocationPositionError};

        let Some(win) = window() else {
            eprintln!("No window object");
            return;
        };

        let Ok(geo) = win.navigator().geolocation() else {
            eprintln!("Geolocation unavailable");
            return;
        };

        let success = Closure::wrap(Box::new(move |pos: GeolocationPosition| {
            let c = pos.coords();
            let coords = Coords {
                lat: c.latitude(),
                lon: c.longitude(),
            };

            spawn(async move {
                if let Err(e) = post_gps(coords).await {
                    eprintln!("Failed to post geolocation: {e}");
                }
            });
        }) as Box<dyn FnMut(_)>);

        let failure = Closure::wrap(Box::new(move |e: GeolocationPositionError| {
            eprintln!("Geolocation error: {}", e.code());
        }) as Box<dyn FnMut(_)>);

        let _ = geo.get_current_position_with_error_callback(
            success.as_ref().unchecked_ref(),
            Some(failure.as_ref().unchecked_ref()),
        );

        success.forget();
        failure.forget();
    }
}
