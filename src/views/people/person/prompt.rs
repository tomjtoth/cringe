use dioxus::prelude::*;

use crate::models::person::PersonPrompt;
use crate::views::people::person::container::Container;

#[component]

pub fn Prompt(prompt: Option<PersonPrompt>, id: Option<i32>) -> Element {
    rsx! {
        if let Some(prompt) = prompt {
            Container { id,
                h3 { class: "p-2 pt-10", "{prompt.title}" }
                p { class: "p-2 pb-20 text-2xl", "{prompt.body}" }
            }
        }
    }
}
