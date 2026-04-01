use dioxus::prelude::*;

use crate::models::person::{Person, RelationshipType as ERT};

#[component]
pub(super) fn RelationshipType(sig: Signal<Person>, editing: bool) -> Element {
    rsx! {
        if editing {
            {
                let value = sig
                    .read()
                    .relationship_type
                    .map(|s| s.to_string())
                    .unwrap_or("".to_string());

                rsx! {
                    div {
                        select {
                            class: if value == "" { "text-gray-500" },
                            value,
                            onchange: move |evt| {
                                sig.write().relationship_type = ERT::from_str(&evt.value());
                            },

                            option { value: "", "👩‍❤️‍👨 Your relationship type..." }
                            option { value: "{ERT::FiguringOutMyRelationshipType}", "{ERT::FiguringOutMyRelationshipType}" }
                            option { value: "{ERT::Monogamy}", "{ERT::Monogamy}" }
                            option { value: "{ERT::NonMonogamy}", "{ERT::NonMonogamy}" }
                        }
                    }
                }
            }
        } else {
            if let Some(relationship_type) = &sig.read().relationship_type {
                {
                    let (emoji, text) = relationship_type.parts();

                    rsx! {
                        div {
                            "{emoji}"
                            div { "{text}" }
                        }
                    }
                }
            }
        }
    }
}
