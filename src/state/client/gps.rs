use std::rc::Rc;

use dioxus::prelude::*;

use crate::models::person::Gps;

#[post("/api/gps")]
async fn post_gps(coords: Gps) -> Result<()> {
    if let (Some(sess_id), pool) = crate::state::server::get_ctx().await {
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
            error!("expired session \"{sess_id}\", nothing to update")
        }
    }

    Ok(())
}

pub(super) fn use_gps_watch() {
    let _gps = use_hook(|| {
        #[cfg(target_arch = "wasm32")]
        {
            use crate::{models::person::Gps, state::client::ME};

            if ME.with(|auth| auth.as_ref().is_some_and(|profile| profile.is_some())) {
                use wasm_bindgen::{closure::Closure, JsCast};
                use web_sys::{window, GeolocationPosition, GeolocationPositionError};

                struct GpsWatch {
                    geo: web_sys::Geolocation,
                    watch_id: i32,
                    _success: Closure<dyn FnMut(GeolocationPosition)>,
                    _failure: Closure<dyn FnMut(GeolocationPositionError)>,
                }

                impl Drop for GpsWatch {
                    fn drop(&mut self) {
                        info!("Stopping GPS watch");
                        self.geo.clear_watch(self.watch_id);
                    }
                }

                let win = window().expect("no window");
                let geo = win.navigator().geolocation().expect("no geolocation");

                let mut last_sent = 0i64;

                let success = Closure::wrap(Box::new(move |pos: GeolocationPosition| {
                    let now = chrono::Utc::now().timestamp();

                    // throttle: 5 min
                    if now < last_sent + 5 * 60 {
                        return;
                    }
                    last_sent = now;

                    let c = pos.coords();
                    let coords = Gps {
                        lat: c.latitude(),
                        lon: c.longitude(),
                    };

                    info!("GPS update: {}, {}", coords.lat, coords.lon);

                    ME.with_mut(|oop| {
                        if let Some(op) = oop.as_mut() {
                            if let Some(p) = op.as_mut() {
                                p.gps = Some(coords.clone())
                            }
                        }
                    });

                    spawn(async move {
                        if let Err(e) = post_gps(coords).await {
                            error!("Failed to post geolocation: {e}");
                        }
                    });
                }) as Box<dyn FnMut(_)>);

                let failure = Closure::wrap(Box::new(move |e: GeolocationPositionError| {
                    error!("Geolocation error: {}", e.code());
                }) as Box<dyn FnMut(_)>);

                let watch_id = geo.watch_position_with_error_callback(
                    success.as_ref().unchecked_ref(),
                    Some(failure.as_ref().unchecked_ref()),
                );

                // 👇 RETURN STATE → persists across renders
                Some(Rc::new(GpsWatch {
                    geo,
                    watch_id,
                    _success: success,
                    _failure: failure,
                }))
            } else {
                None
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        None::<Rc<()>>
    });
}
