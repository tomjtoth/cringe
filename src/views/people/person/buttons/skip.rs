use dioxus::prelude::*;

use crate::{models::person::Decision, state::client::DECISIONS};

#[component]
pub fn SkipButton(id: Option<i32>) -> Element {
    rsx! {
        if let Some(id) = id {
            if !DECISIONS.with(|ds| matches!(ds.get(&id), Some(&Decision::Skip))) {
                button {
                    class: "z-2 sticky bottom-10 left-5 p-3!
                        bg-background rounded-full!
                        cursor-pointer select-none",

                    onclick: move |_| {
                        DECISIONS.write().insert(id, Decision::Skip);
                    },
                    "❌"
                }
            }
        }

    }
}
