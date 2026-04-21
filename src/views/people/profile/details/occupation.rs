use dioxus::prelude::*;

use crate::state::ME;
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
                        let val = evt.value();
                        ME.with_mut(|me| {
                            me.draft.as_mut().unwrap().occupation = if val.len() > 0 {
                                Some(val)
                            } else {
                                None
                            };
                        })
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
