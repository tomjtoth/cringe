use dioxus::prelude::*;

use crate::views::people::profile::{details::DetailsCtx, ResourceCtx};

#[component]
pub(super) fn Occupation() -> Element {
    let mut dcx = use_context::<DetailsCtx>();
    let rcx = use_context::<ResourceCtx>();

    rsx! {
        if rcx.editing() {
            div {
                "💼"
                input {
                    placeholder: "Occupation",
                    value: dcx.rw.read().occupation.clone(),
                    onchange: move |evt| {
                        let val = evt.value();
                        dcx.rw.write().occupation = if val.len() > 0 { Some(val) } else { None };
                    },
                }
            }
        } else {
            if let Some(job) = &dcx.ro.read().occupation {
                div {
                    "💼"
                    div { "{job}" }
                }
            }
        }
    }
}
