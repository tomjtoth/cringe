use dioxus::prelude::*;
use tracing_subscriber::{prelude::*, EnvFilter};

use crate::state::client::init_client_state;

#[cfg(feature = "server")]
mod auth;

mod models;
mod navbar;
mod router;
mod state;
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
        use crate::state::server::{init_converter, seed_bots};

        dotenvy::dotenv().ok();

        let pool = sqlx::PgPool::connect(&std::env::var("DATABASE_URL")?).await?;

        sqlx::migrate!().run(&pool).await?;

        if let Err(e) = seed_bots(&pool).await {
            error!("Failed to load bots.yaml: {}", e);
        }

        let tx = init_converter(pool.clone());

        let app = auth::routes(pool.clone())?
            .fallback_service(dioxus::server::router(App))
            .layer(axum::Extension(pool))
            .layer(axum::Extension(tx))
            .layer(axum::extract::DefaultBodyLimit::max(20 * 1024 * 1024));

        Ok(app)
    })
}

// TODO: refactor components under src/ui/{router.rs,navbar.rs,views/{swipe/mod.rs,me/mod.rs}} etc.

#[component]
fn App() -> Element {
    init_client_state()?;

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "icon", href: "/icon.png" }
        document::Link { rel: "manifest", href: "/manifest.json" }

        Router::<router::Route> {}
    }
}
