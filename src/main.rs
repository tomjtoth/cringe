use dioxus::prelude::*;

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
    dioxus::serve(|| async { Ok(dioxus::server::router(App)) })
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<router::Route> {}
    }
}
