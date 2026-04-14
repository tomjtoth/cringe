use dioxus::prelude::*;

use crate::models::person::Person;

#[component]
pub(super) fn Education(sig: Signal<Person>, editing: bool) -> Element {
    rsx! {
        if editing {
            div {
                "🎓"
                input {
                    placeholder: "Education",
                    value: sig.read().education.clone(),
                    onchange: move |evt| {
                        let val = evt.value();
                        sig.write().education = if val.len() > 0 { Some(val) } else { None };
                    },
                }
            }
        } else {
            if let Some(edu) = &sig.read().education {
                div {
                    "🎓"
                    div { "{edu}" }
                }
            }
        }
    }
}
