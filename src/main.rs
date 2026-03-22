use dioxus::prelude::*;

use crate::state::client::{get_decisions, get_me, AuthResponse, DECISIONS, ME};

#[cfg(feature = "server")]
mod auth;

mod models;
mod navbar;
mod router;
mod state;
mod views;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async {
        dotenvy::dotenv().ok();

        let pool = sqlx::PgPool::connect(&std::env::var("DATABASE_URL")?).await?;

        sqlx::migrate!().run(&pool).await?;

        if let Err(e) = crate::state::server::seed_bots(&pool).await {
            eprintln!("Failed to load bots.yaml: {}", e);
        }

        let app = dioxus::server::router(App)
            .merge(auth::routes(pool.clone())?)
            .layer(axum::Extension(pool));
        Ok(app)
    })
}

#[component]
fn App() -> Element {
    let mut initial_state =
        use_server_future(|| async { futures::join!(get_decisions(), get_me()) })?;

    if let Some((decisions, user)) = initial_state.write().as_mut() {
        if let Ok(decisions) = decisions {
            DECISIONS.write().extend(decisions.to_owned());
        }

        if let Ok(AuthResponse(authorized, profile)) = user {
            *ME.write() = if authorized.to_owned() {
                Some(profile.to_owned())
            } else {
                None
            };
        }
    }

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "icon", href: "/icon.png" }
        document::Link { rel: "manifest", href: "/manifest.json" }

        Router::<router::Route> {}
    }
}
