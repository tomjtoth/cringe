use dioxus::prelude::*;
mod models;
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
fn main() {
    dioxus::launch(App);
}
#[component]
fn App() -> Element {

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
    }
}
