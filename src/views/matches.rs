use dioxus::prelude::*;

#[component]
pub fn Matches() -> Element {
    rsx! {
        div { class: "app-center",
            h1 { "TODO" }
        }
    }
}
