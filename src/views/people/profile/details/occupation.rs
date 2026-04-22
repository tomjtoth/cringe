use dioxus::prelude::*;

use crate::state::{TrMe, ME};
use crate::views::people::profile::{ProfileCtx, ResourceCtx};

#[component]
pub(super) fn Occupation() -> Element {
    let pcx = use_context::<ProfileCtx>();
    let rcx = use_context::<ResourceCtx>();

    rsx! {
        if rcx.editing() {
            div {
                "💼"
                input {
                    placeholder: "Occupation",
                    value: ME.with(|me| me.draft.as_ref().and_then(|p| p.occupation.clone())),
                    onchange: move |evt| {
                        ME.mut_draft(|d| d.occupation = Some(evt.value()).filter(|v| v.len() > 0))
                    },
                }
            }
        } else {
            if let Some(job) = &pcx.read().occupation {
                div {
                    "💼"
                    div { "{job}" }
                }
            }
        }
    }
}
