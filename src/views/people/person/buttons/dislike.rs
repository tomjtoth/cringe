use dioxus::prelude::*;

use crate::{models::person::Liked, state::client::PEEPS};

#[component]
pub fn DislikeButton(id: i32) -> Element {
    let person_liked = PEEPS.iter().find(|p| p.id == Some(id)).unwrap().liked;

    rsx! {
        if !matches!(person_liked, Some(Liked::No)) {
            button {
                class: "z-2 sticky bottom-10 left-5 p-3
                        bg-background border rounded-full
                        cursor-pointer select-none",

                onclick: move |_| {
                    for p in PEEPS.write().iter_mut() {
                        if p.id == Some(id) {
                            p.liked = Some(Liked::No);
                            break;
                        }
                    }
                },
                "❌"
            }
        }

    }
}
