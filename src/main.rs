use dioxus::prelude::*;

use crate::state::client::use_state_initializer;

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
        use sqlx::PgPool;

        let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;

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
    use_state_initializer();

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "icon", href: "/icon.png" }
        document::Link { rel: "manifest", href: "/manifest.json" }

        Router::<router::Route> {}
    }
}
