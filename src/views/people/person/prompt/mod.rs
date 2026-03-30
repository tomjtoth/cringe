use dioxus::prelude::*;

use crate::models::person::PersonPrompt as MPrompt;

use crate::views::people::listing::ListingCtx;
use crate::views::people::person::prompt::editor::PromptEditor;
use crate::views::people::person::{container::Container, ResourceCtx};

mod editor;

#[component]
pub fn Prompt(src: Option<MPrompt>) -> Element {
    let olcx = use_context::<Option<ListingCtx>>();
    let mut rcx = ResourceCtx::provide();

    rsx! {
        if let Some(prompt) = &src {
            if rcx.editing() {
                PromptEditor { src }
            } else {
                Container { class: "pt-10 pb-20 px-2",
                    h3 { class: "p-2", "{prompt.title}" }
                    h2 { class: "p-2", "{prompt.body}" }
                }
            }
        } else {
            if olcx.is_none() {
                if rcx.editing() {
                    PromptEditor {}
                } else {
                    Container {
                        class: "p-2 px-5 flex gap-2 justify-between items-center",
                        wo_button: true,

                        "Add a prompt here"
                        button {
                            class: "border-2!",
                            onclick: move |_| rcx.next_state(),
                            "➕"
                        }
                    }
                }
            }
        }
    }
}
