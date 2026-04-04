use dioxus::prelude::*;

use crate::views::people::person::{container::Container, ResourceCtx};

#[component]
pub(super) fn Adder(what: String) -> Element {
    let mut rcx = use_context::<ResourceCtx>();

    rsx! {
        Container {
            class: "p-2 px-5 flex gap-2 justify-between items-center",
            wo_button: true,

            "Add {what} here"
            button { class: "border-2!", onclick: move |_| rcx.next_state(), "➕" }
        }
    }
}
