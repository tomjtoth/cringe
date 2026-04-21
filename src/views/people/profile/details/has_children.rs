use dioxus::prelude::*;

use crate::state::ME;
use crate::views::people::profile::{ProfileCtx, ResourceCtx};

#[component]
pub(super) fn HasChildren() -> Element {
    let pcx = use_context::<ProfileCtx>();
    let rcx = use_context::<ResourceCtx>();

    let value = ME.with(|me| me.draft.as_ref().and_then(|p| p.has_children));

    rsx! {
        if rcx.editing() {
            li {
                "🧑‍🧒‍🧒"
                select {
                    class: if value.is_none() { "text-gray-500" },
                    onchange: move |evt| {
                        let parsed = evt.value().parse::<bool>().ok();
                        ME.with_mut(|me| me.draft.as_mut().unwrap().has_children = parsed)
                    },

                    option { value: "", "Any children?" }
                    option { value: false, selected: value == Some(false), "I don't have children" }
                    option { value: true, selected: value == Some(true), "I have children" }
                }
            }
        } else {
            if let Some(has) = pcx.read().has_children {
                li {
                    "🧑‍🧒‍🧒 Has"
                    if !has {
                        " no"
                    }
                    " children"
                }
            }
        }
    }
}
