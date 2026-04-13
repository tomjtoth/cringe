use dioxus::prelude::*;

use crate::state::init_client;

#[cfg(feature = "server")]
mod auth;

mod models;
mod navbar;
mod router;
mod state;
mod utils;
mod views;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async {
        let pool = crate::state::server::init().await?;

        // TODO: stop passing pool.clone to auth::routes,
        // make use of Dioxus' extractor style if possible
        let app = auth::routes(pool.clone())?
            .fallback_service(dioxus::server::router(App))
            .layer(axum::Extension(pool));

        Ok(app)
    })
}

// TODO: refactor components under src/ui/{router.rs,navbar.rs,views/{swipe/mod.rs,me/mod.rs}} etc.

#[component]
fn App() -> Element {
    init_client()?;

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "icon", href: "/icon.png" }
        document::Link { rel: "manifest", href: "/manifest.json" }

        Router::<router::Route> {}
    }
}
