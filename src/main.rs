use dioxus::prelude::*;
mod models;
mod router;
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
fn main() {
    dioxus::launch(App);
}
#[component]
fn App() -> Element {

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<router::Route> {}
    }
}
