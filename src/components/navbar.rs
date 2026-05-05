use dioxus::prelude::*;

use crate::{components::router::Route, state::ME};

#[component]
pub fn Navbar() -> Element {
    let me = ME.read();

    rsx! {
        ul { class: "py-2 bg-background border-t flex items-center justify-around w-full
                     [&_a]:flex [&_a]:flex-col text-center",

            li {
                Link { to: Route::Cringe {},
                    "😬"
                    span { "cringe" }
                }
            }

            li {
                Link { to: Route::Matches {},
                    "💬"
                    span { "matches" }
                }
            }

            li {
                Link { to: Route::Me {},
                    if let Some(image) = me.profile.as_ref().and_then(|p| p.images.get(0)) {
                        img {
                            class: "w-6 border rounded-full",
                            src: image.src(),
                        }
                    } else {
                        "🧑"
                        span { "profile" }
                    }
                }
            }
        }
    }
}
