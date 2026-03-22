use dioxus::prelude::*;

use crate::{router::Route, state::client::ME};

#[component]
pub fn Navbar() -> Element {
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
                    "💔"
                    span { "skipped" }
                }
            }
            li {
                Link { to: Route::SwipeProfiles {},
                    "🎉"
                    span { "swipe" }
                }
            }
            li {
                Link { to: Route::LikedProfiles {},
                    "❤️"
                    span { "liked" }
                }
            }
            li {
                if let Some(logged_in) = ME() {
                    Link { to: Route::Me {},
                        if let Some(pic) = logged_in.as_ref().and_then(|p| p.pics().get(0)) {
                            img {
                                class: "w-6 border rounded-full",
                                src: pic.src(),
                            }
                        } else {
                            "🧑"
                            span { "profile" }
                        }
                    }
                } else {
                    Link { to: "/auth/discord",

                        "➜]"
                        span { "login" }
                    }
                }
            }
        }
    }
}
