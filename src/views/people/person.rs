use dioxus::prelude::*;

use crate::{
    models::person::{Liked, Pic},
    state::client::PEEPS,
};

#[component]
pub fn Person(id: String) -> Element {
    let person = PEEPS.iter().find(|p| p.id == id).unwrap();

    let pics = &person.pictures;

    rsx! {
        ul { class: "overflow-scroll h-full",
            for pic in pics {
                img {
                    class: "object-cover",
                    src: match pic {
                        Pic::Url(src) => src.clone(),
                        Pic::Advanced { url, .. } => url.clone(),
                    },
                    alt: "broken link to picture",
                }
            }
        }

        h2 { "{person.name}, {person.age()}" }

        if !matches!(person.liked, Some(Liked::Yes)) {
            button {
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

        if !matches!(person.liked, Some(Liked::No)) {
            button {
                onclick: move |_| {
                    for p in PEEPS.write().iter_mut() {
                        if p.id == person.id {
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
