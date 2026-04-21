use dioxus::prelude::*;

use crate::views::people::profile::{ProfileCtx, ResourceCtx};
use crate::state::ME;

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
                    onchange: move |evt| {
                        let loc = evt.value();
                        ME.with_mut(|me| {
                            me.draft.as_mut().unwrap().location = if loc.len() > 0 {
                                Some(loc)
                            } else {
                                None
                            };
                        })
                    },
                }
            }
        } else {
            if let Some(city) = &pcx.read().location {
                li { "📍 {city}" }
            }
        }
    }
}
