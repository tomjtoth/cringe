use dioxus::prelude::*;

use crate::views::people::person::container::Container;

#[component]

pub fn Prompt(pp: Option<(String, String)>) -> Element {
    rsx! {
        if let Some((prompt, text)) = pp {
            Container {
                h3 { class: "p-2 pt-10", "{prompt}" }
                p { class: "p-2 pb-10 text-2xl", "{text}" }
            }
        }
    }
}
