use dioxus::prelude::*;

use crate::models::{person::Person, relationship_type::RelationshipType as ERT};

#[component]
pub(super) fn RelationshipType(sig: Signal<Person>, editing: bool) -> Element {
    let tmp = sig.read().relationship_type;
    let r = tmp.as_ref();
    let value = r.map(|rt| rt.to_string());

    rsx! {
        if editing {
            div {
                select {
                    class: if value == None { "text-gray-500" },
                    value,
                    onchange: move |evt| {
                        sig.write().relationship_type = ERT::from_str(&evt.value());
                    },

                    option { value: "", "👩‍❤️‍👨 Your relationship type..." }
                    option { value: "{ERT::FiguringOutMyRelationshipType}",
                        "{ERT::FiguringOutMyRelationshipType}"
                    }
                    option { value: "{ERT::Monogamy}", "{ERT::Monogamy}" }
                    option { value: "{ERT::NonMonogamy}", "{ERT::NonMonogamy}" }
                }
            }
        } else {
            if let Some((emoji, text)) = r.map(|rt| rt.parts()) {
                div {
                    "{emoji}"
                    div { "{text}" }
                }
            }
        }
    }
}
