use dioxus::prelude::*;

use crate::models::person::Person;

pub static PEEPS: GlobalSignal<Vec<Person>> = Signal::global(|| vec![]);

#[get("/bots")]
async fn get_bots() -> Result<Vec<Person>> {
    Ok(super::server::BOTS.clone())
}

pub fn use_bot_loader() {
    let _ = use_server_future(|| async {
        if let Ok(mut bots) = get_bots().await {
            PEEPS.write().append(&mut bots);
            println!("writing peeps succeeded!");
        } else {
            eprintln!("writing peeps failed!");
        }
    });
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Coords {
    lat: f64,
    lon: f64,
}

#[component]
fn GeoExample() -> Element {
    let mut coords = use_signal(|| None::<Coords>);
    let mut err = use_signal(|| None::<String>);

    let request_location = move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::{closure::Closure, JsCast};
            use web_sys::{window, GeolocationPosition, GeolocationPositionError};

            let Some(win) = window() else {
                *err.write() = Some("No window object".to_string());
                return;
            };

            let Ok(geo) = win.navigator().geolocation() else {
                *err.write() = Some("Geolocation unavailable".to_string());
                return;
            };

            let mut coords_sig = coords;
            let success = Closure::wrap(Box::new(move |pos: GeolocationPosition| {
                let c = pos.coords();
                *coords_sig.write() = Some(Coords {
                    lat: c.latitude(),
                    lon: c.longitude(),
                });
            }) as Box<dyn FnMut(_)>);

            let mut err_sig = err;
            let failure = Closure::wrap(Box::new(move |e: GeolocationPositionError| {
                *err_sig.write() = Some(format!("Geolocation error: {}", e.code()));
            }) as Box<dyn FnMut(_)>);

            let _ = geo.get_current_position_with_error_callback(
                success.as_ref().unchecked_ref(),
                Some(failure.as_ref().unchecked_ref()),
            );

            success.forget();
            failure.forget();
        }
    };

    rsx! {
        button { onclick: request_location, "Get location" }
        if let Some(c) = coords() {
            p { "lat: {c.lat}, lon: {c.lon}" }
        }
        if let Some(e) = err() {
            p { "error: {e}" }
        }
    }
}
