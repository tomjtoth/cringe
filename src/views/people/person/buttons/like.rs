use dioxus::prelude::*;

use crate::{models::person::Decision, state::client::DECISIONS};

#[component]
pub fn LikeButton(id: i32) -> Element {
    let liked_already =
        DECISIONS.with(|decisions| matches!(decisions.get(&id), Some(&Decision::Like)));

    rsx! {
        if !liked_already {
            button {
                class: "absolute z-2 bottom-5 right-5 p-3!
                        bg-background rounded-full!
                        cursor-pointer select-none",

                onclick: move |_| {
                    DECISIONS.write().insert(id, Decision::Like);
                },
                "❤️"
            }
        }
    }
}
