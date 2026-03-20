use std::collections::HashMap;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use crate::state::server::{get_db, get_session_id};

/// **outer** option indicates an **authorized session**
///
/// **inner** option holds the **user profile** if exists
pub static ME: GlobalSignal<Option<Option<Person>>> = Signal::global(|| None);

pub static PEEPS: GlobalSignal<Vec<Person>> = Signal::global(|| vec![]);

pub static DECISIONS: GlobalSignal<HashMap<i32, Decision>> = Signal::global(|| HashMap::new());

#[get("/api/decisions")]
pub async fn get_decisions() -> Result<Vec<(i32, Decision)>> {
    #[cfg(feature = "server")]
    {
        if let Some(session_id) = get_session_id().await {
            let pool = get_db().await;

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
    }

    Ok(vec![])
}

#[get("/api/me/pic")]
pub async fn get_my_pic() -> Result<Option<Pic>> {
    #[cfg(feature = "server")]
    {
        let cookies = crate::state::server::get_cookies().await;

        let session_id = match cookies.get("SESSION") {
            Some(v) => v,
            None => return Ok(None),
        };

        let pool = crate::state::server::get_db().await;

        // Get user's primary avatar URL (if any)
        let pic_parts: Option<(String, Vec<u8>)> = sqlx::query_scalar(
            "
            SELECT up.mime_type, up.image_bytes
            FROM auth_sessions a
            JOIN users u ON u.email = a.email
            JOIN user_pictures up ON up.user_id = u.id
            WHERE a.id = $1 AND a.expires_at > NOW()
            ORDER BY up.position
            LIMIT 1
            ",
        )
        .bind(session_id)
        .fetch_optional(&pool)
        .await?;

        let Some((mime_type, bytes)) = pic_parts else {
            return Ok(None);
        };

        let pic = Pic::Uploaded {
            bytes,
            mime_type,
            prompt: None,
        };

        Ok(Some(pic))
    }
    #[cfg(not(feature = "server"))]
    Ok(None)
}

pub fn use_state_initializer() {
    update_coords();
    let _ = use_server_future(|| async {
        if let Ok(decisions) = get_decisions().await {
            DECISIONS.write().extend(decisions);
            println!("writing decisions succeeded!");
        } else {
            eprintln!("writing decisions failed!");
        }
    });
}

#[post("/geo")]
async fn post_geo_location(coords: Coords) -> Result<()> {
    let _store_these_in_db_later = coords;

    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
struct Coords {
    lat: f64,
    lon: f64,
}

fn update_coords() {
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
                if let Err(e) = post_geo_location(coords).await {
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
