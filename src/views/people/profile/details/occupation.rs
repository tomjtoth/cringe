use dioxus::prelude::*;

use crate::models::person::Person;

#[component]
pub(super) fn Occupation(sig: Signal<Person>, editing: bool) -> Element {
    rsx! {
        if editing {
            div {
                "💼"
                input {
                    placeholder: "Occupation",
                    value: sig.read().occupation.clone(),
                    onchange: move |evt| {
                        let val = evt.value();
                        sig.write().occupation = if val.len() > 0 { Some(val) } else { None };
                    },
                }
            }
        } else {
            if let Some(job) = &sig.read().occupation {
                div {
                    "💼"
                    div { "{job}" }
                }
            }
        }
    }
}
