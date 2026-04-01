use dioxus::prelude::*;

use crate::models::person::{Person, Seeking as ES};

#[component]
pub(super) fn Seeking(sig: Signal<Person>, editing: bool) -> Element {
    rsx! {
        if editing {
            {
                let value = sig
                    .read()
                    .seeking
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or("".to_string());

                rsx! {
                    div {

                        select {
                            class: if value == "" { "text-gray-500" },

                            value,
                            onchange: move |evt| {
                                sig.write().seeking = ES::from_str(&evt.value());
                            },

                            option { value: "", class: "text-gray-500", "🕵️ You're seeking..." }
                            option { value: "{ES::LongTerm}", "{ES::LongTerm}" }
                            option { value: "{ES::LongTermOpenToShort}", "{ES::LongTermOpenToShort}" }
                            option { value: "{ES::ShortTermFun}", "{ES::ShortTermFun}" }
                            option { value: "{ES::ShortTermOpenToLong}", "{ES::ShortTermOpenToLong}" }
                            option { value: "{ES::StillFiguringItOut}", "{ES::StillFiguringItOut}" }
                        }
                    }
                }
            }
        } else {
            if let Some(seeking) = &sig.read().seeking {
                {
                    let (emoji, text) = seeking.parts();

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
