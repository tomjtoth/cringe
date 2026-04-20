use dioxus::prelude::*;

use crate::{router::Route, state::ME};

#[component]
pub fn Navbar() -> Element {
    let me = ME.read();

    rsx! {
        div { class: "grow overflow-hidden", Outlet::<Route> {} }

        ul { class: "py-2 bg-background border-t flex items-center justify-around w-full
                     [&_a]:flex [&_a]:flex-col text-center",
            li {
                Link { to: Route::About {},
                    "⁉️"
                    span { "about" }
                }
            }
            li {
                Link { to: Route::SkippedProfiles {},
                    "🚫"
                    span { "skipped" }
                }
            }
            li {
                Link { to: Route::SwipeProfiles {},
                    "😬"
                    span { "cringe" }
                }
            }
            li {
                Link { to: Route::LikedProfiles {},
                    "✅"
                    span { "liked" }
                }
            }
            li {
                if me.authenticated {
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
                } else {
                    Link { to: Route::Login {},
                        "➜]"
                        span { "login" }
                    }
                }
            }
        }
    }
}
