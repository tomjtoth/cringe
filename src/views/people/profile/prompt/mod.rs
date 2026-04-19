use dioxus::prelude::*;

use crate::views::people::listing::ListingCtx;
use crate::views::people::profile::{
    container::Container, prompt::editor::PromptEditor, ProfileCtx, ResourceCtx,
};

mod editor;

#[component]
pub fn Prompt(idx: usize) -> Element {
    let olcx = use_context::<Option<ListingCtx>>();
    let rcx = ResourceCtx::provide(1 + idx);

    let (src, show_adder) = {
        let pcx = use_context::<ProfileCtx>();
        let person = (pcx.profile)();
        let prompts = person.prompts();
        let op = prompts.get(idx);

        (op.cloned(), idx == prompts.len())
    };

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
            if olcx.is_none() && show_adder {
                if rcx.editing() {
                    PromptEditor {}
                } else {
                    Container { class: "p-2 px-5 flex gap-2 justify-between items-center [&_button]:static",

                        "Add a new prompt"
                    }
                }
            }
        }
    }
}
