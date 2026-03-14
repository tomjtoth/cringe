use dioxus::prelude::*;

use crate::{models::person::Liked, state::client::PEEPS};

#[component]
pub fn LikeButton(id: String) -> Element {
    let person_liked = PEEPS.iter().find(|p| p.id == id).unwrap().liked;

    rsx! {
        if !matches!(person_liked, Some(Liked::Yes)) {
            button {
                class: "absolute z-2 bottom-5 right-5 p-2
                        bg-background border rounded-full
                        cursor-pointer select-none",

                onclick: move |_| {
                    for p in PEEPS.write().iter_mut() {
                        if p.id == id {
                            p.liked = Some(Liked::Yes);
                            break;
                        }
                    }
                },
                "✅"
            }
        }
    }
}
