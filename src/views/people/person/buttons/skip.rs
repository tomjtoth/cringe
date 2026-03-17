use dioxus::prelude::*;

use crate::{models::person::Decision, state::client::DECISIONS};

#[component]
pub fn SkipButton(id: i32) -> Element {
    let skipped_already =
        DECISIONS.with(|decisions| matches!(decisions.get(&id), Some(&Decision::Skip)));

    rsx! {
        if !skipped_already {
            button {
                class: "z-2 sticky bottom-10 left-5 p-3
                        bg-background border rounded-full
                        cursor-pointer select-none",

                onclick: move |_| {
                    DECISIONS.write().insert(id, Decision::Skip);
                },
                "❌"
            }
        }

    }
}
