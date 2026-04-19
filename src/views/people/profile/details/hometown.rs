use dioxus::prelude::*;

use crate::views::people::profile::details::DetailsCtx;

#[component]
pub(super) fn Hometown() -> Element {
    let mut dcx = use_context::<DetailsCtx>();

    rsx! {
        if (dcx.editing)() {
            div {
                "🏠"
                input {
                    placeholder: "Hometown",
                    value: dcx.rw.read().hometown.clone(),
                    onchange: move |evt| {
                        let val = evt.value();
                        dcx.rw.write().hometown = if val.len() > 0 { Some(val) } else { None };
                    },
                }
            }
        } else {
            if let Some(ht) = &dcx.ro.read().hometown {
                div {
                    "🏠"
                    div { "{ht}" }
                }
            }
        }
    }
}
