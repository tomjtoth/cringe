use dioxus::prelude::*;

use crate::models::person::Person;

#[component]
pub(super) fn Location(sig: Signal<Person>, editing: bool) -> Element {
    rsx! {
        if editing {
            li {
                "📍"
                input {
                    placeholder: "Location",
                    class: "w-30",
                    value: sig.read().location.clone(),
                    onchange: move |evt| {
                        let loc = evt.value();
                        sig.write().location = if loc.len() > 0 { Some(loc) } else { None };
                    },
                }
            }
        } else {
            if let Some(city) = &sig.read().location {
                li { "📍 {city}" }
            }
        }
    }
}
