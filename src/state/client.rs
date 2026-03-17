use std::collections::HashMap;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::person::{Decision, Person};

pub static PEEPS: GlobalSignal<Vec<Person>> = Signal::global(|| vec![]);

pub static DECISIONS: GlobalSignal<HashMap<i32, Decision>> = Signal::global(|| HashMap::new());

#[get("/api/decisions")]
async fn get_decisions() -> Result<Vec<(i32, Decision)>> {
    #[cfg(feature = "server")]
    {
        let pool = crate::state::server::get_db().await;

        // Demo query: fetch recent checks as (target_user_id, decision).
        let decisions = sqlx::query_as::<_, (i32, Decision)>(
            "SELECT user_id2, decision FROM user_checks ORDER BY updated_at DESC LIMIT 500",
        )
        .fetch_all(&pool)
        .await?;

        Ok(decisions)
    }

    #[cfg(not(feature = "server"))]
    Ok(vec![])
}

pub fn use_state_initializer() {
    let _ = use_server_future(|| async {
        if let Ok(decisions) = get_decisions().await {
            DECISIONS.write().extend(decisions);
            println!("writing peeps succeeded!");
        } else {
            eprintln!("writing peeps failed!");
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

pub fn update_coords() {
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
