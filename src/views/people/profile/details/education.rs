use dioxus::prelude::*;

use crate::views::people::profile::details::DetailsCtx;

#[component]
pub(super) fn Education() -> Element {
    let mut dcx = use_context::<DetailsCtx>();

    rsx! {
        if (dcx.editing)() {
            div {
                "🎓"
                input {
                    placeholder: "Education",
                    value: dcx.rw.read().education.clone(),
                    onchange: move |evt| {
                        let val = evt.value();
                        dcx.rw.write().education = if val.len() > 0 { Some(val) } else { None };
                    },
                }
            }
        } else {
            if let Some(edu) = &dcx.ro.read().education {
                div {
                    "🎓"
                    div { "{edu}" }
                }
            }
        }
    }
}
