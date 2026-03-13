use dioxus::prelude::*;

use crate::{models::person::Liked, state::client::PEEPS};

#[component]
pub fn DislikeButton(id: String, person_liked: Option<Liked>) -> Element {
    rsx! {

        if !matches!(person_liked, Some(Liked::No)) {
            button {
                class: "z-2 absolute bottom-20 left-5 border rounded",
                onclick: move |_| {
                    for p in PEEPS.write().iter_mut() {
                        if p.id == id {
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
