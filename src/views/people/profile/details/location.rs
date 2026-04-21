use dioxus::prelude::*;

use crate::views::people::profile::{details::DetailsCtx, ResourceCtx};

#[component]
pub(super) fn Location() -> Element {
    let mut dcx = use_context::<DetailsCtx>();
    let rcx = use_context::<ResourceCtx>();

    rsx! {
        if rcx.editing() {
            li {
                "📍"
                input {
                    placeholder: "Location",
                    class: "w-30",
                    value: dcx.rw.read().location.clone(),
                    onchange: move |evt| {
                        let loc = evt.value();
                        dcx.rw.write().location = if loc.len() > 0 { Some(loc) } else { None };
                    },
                }
            }
        } else {
            if let Some(city) = &dcx.ro.read().location {
                li { "📍 {city}" }
            }
        }
    }
}
