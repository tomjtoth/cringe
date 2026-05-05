use dioxus::prelude::*;

use crate::components::profile::{ProfileCtx, ResourceCtx};
use crate::state::{TrMe, ME};

#[component]
pub(super) fn Hometown() -> Element {
    let pcx = use_context::<ProfileCtx>();
    let rcx = use_context::<ResourceCtx>();

    rsx! {
        if rcx.editing() {
            div {
                "🏠"
                input {
                    placeholder: "Hometown",
                    value: ME.with(|me| me.draft.as_ref().and_then(|p| p.hometown.clone())),
                    onchange: move |evt| ME.mut_draft(|d| d.hometown = Some(evt.value()).filter(|s| s.len() > 0)),
                }
            }
        } else {
            if let Some(ht) = &pcx.read().hometown {
                div {
                    "🏠"
                    div { "{ht}" }
                }
            }
        }
    }
}
