use dioxus::prelude::*;

use crate::{models::person::Liked, state::client::PEEPS};

#[component]
pub fn LikeButton(id: String, person_liked: Option<Liked>) -> Element {
    rsx! {
        if !matches!(person_liked, Some(Liked::Yes)) {
            button {
                class: "z-2 bottom-5 right-5",
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
