use dioxus::prelude::*;

use crate::views::people::profile::{ProfileCtx, ResourceCtx};
use crate::state::ME;

#[component]
pub(super) fn Education() -> Element {
    let pcx = use_context::<ProfileCtx>();
    let rcx = use_context::<ResourceCtx>();

    rsx! {
        if rcx.editing() {
            div {
                "🎓"
                input {
                    placeholder: "Education",
                    value: ME.with(|me| me.draft.as_ref().and_then(|p| p.education.clone())),
                    onchange: move |evt| {
                        let val = evt.value();
                        ME.with_mut(|me| {
                            me.draft.as_mut().unwrap().education = if val.len() > 0 {
                                Some(val)
                            } else {
                                None
                            };
                        })
                    },
                }
            }
        } else {
            if let Some(edu) = &pcx.read().education {
                div {
                    "🎓"
                    div { "{edu}" }
                }
            }
        }
    }
}
