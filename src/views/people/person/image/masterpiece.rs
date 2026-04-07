use dioxus::prelude::*;

#[component]
pub(super) fn Masterpiece() -> Element {
    rsx! {

        div {
            class: "border-15 border-amber-300 dark:border-amber-700 border-ridge",
            class: "px-5 py-30 relative select-none text-center",

            span { class: "text-9xl ", "🧑" }
        }
    }
}
