use dioxus::prelude::*;

use crate::state::client::init_client_state;

#[cfg(feature = "server")]
mod auth;

mod models;
mod navbar;
mod router;
mod state;
mod views;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        use tracing_subscriber::{prelude::*, EnvFilter};

        let filter = EnvFilter::new(
            "info,\
             dioxus=warn,\
             dioxus_core=warn,\
             dioxus_web=warn,\
             dioxus_signals=warn,\
             hyper=warn,\
             mio=warn",
        );

        tracing_subscriber::registry()
            .with(filter)
            .with(tracing_wasm::WASMLayer::default())
            .init();
    }

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async {
        dotenvy::dotenv().ok();

        let pool = sqlx::PgPool::connect(&std::env::var("DATABASE_URL")?).await?;

        sqlx::migrate!().run(&pool).await?;

        if let Err(e) = crate::state::server::seed_bots(&pool).await {
            error!("Failed to load bots.yaml: {}", e);
        }

        let app = dioxus::server::router(App)
            .merge(auth::routes(pool.clone())?)
            .layer(axum::Extension(pool));

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
