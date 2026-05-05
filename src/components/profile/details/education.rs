use dioxus::prelude::*;

use crate::components::profile::{ProfileCtx, ResourceCtx};
use crate::state::{TrMe, ME};

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
                    onchange: move |evt| ME.mut_draft(|d| d.education = Some(evt.value()).filter(|v| v.len() > 0)),
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
