use dioxus::prelude::*;

use crate::models::person::Person;

#[component]
pub(super) fn Hometown(sig: Signal<Person>, editing: bool) -> Element {
    rsx! {
        if editing {
            div {
                "🏠"
                input {
                    placeholder: "Hometown",
                    value: sig.read().hometown.clone(),
                    onchange: move |evt| {
                        let val = evt.value();
                        sig.write().hometown = if val.len() > 0 { Some(val) } else { None };
                    },
                }
            }
        } else {
            if let Some(ht) = &sig.read().hometown {
                div {
                    "🏠"
                    div { "{ht}" }
                }
            }
        }
    }
}
