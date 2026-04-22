use dioxus::prelude::*;

use crate::state::{TrMe, ME};
use crate::views::people::profile::{ProfileCtx, ResourceCtx};

#[component]
pub(super) fn Location() -> Element {
    let pcx = use_context::<ProfileCtx>();
    let rcx = use_context::<ResourceCtx>();

    rsx! {
        if rcx.editing() {
            li {
                "📍"
                input {
                    placeholder: "Location",
                    class: "w-30",
                    value: ME.with(|me| me.draft.as_ref().and_then(|p| p.location.clone())),
                    onchange: move |evt| ME.mut_draft(|d| d.location = Some(evt.value()).filter(|l| l.len() > 0)),
                }
            }
        } else {
            if let Some(city) = &pcx.read().location {
                li { "📍 {city}" }
            }
        }
    }
}
