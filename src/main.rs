use dioxus::prelude::*;
use tracing_subscriber::{prelude::*, EnvFilter};

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

fn tracing_sub() {
    let filter = EnvFilter::new(
        "info,\
             dioxus=warn,\
             dioxus_core=warn,\
             dioxus_web=warn,\
             dioxus_signals=warn,\
             hyper=warn,\
             mio=warn,\
             sqlx=warn",
    );

    let layer = {
        #[cfg(target_arch = "wasm32")]
        {
            tracing_wasm::WASMLayer::default()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            tracing_subscriber::fmt::layer()
        }
    };

    tracing_subscriber::registry()
        .with(filter)
        .with(layer)
        .init();
}

fn main() {
    tracing_sub();

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async {
        let (pool, tx) = crate::state::server::init().await?;

        // TODO: stop passing pool.clone to auth::routes,
        // make use of Dioxus' extractor style if possible
        let app = auth::routes(pool.clone())?
            .fallback_service(dioxus::server::router(App))
            .layer(axum::Extension(pool))
            .layer(axum::Extension(tx));

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
