use dioxus::prelude::*;

use crate::{router::Route, state::client::use_bot_loader};

#[component]
pub fn Navbar() -> Element {
    use_bot_loader();

    rsx! {
        div { class: "grow overflow-scroll", Outlet::<Route> {} }

        ul { class: "bg-background border-t flex justify-around w-full
                     [&_a]:flex [&_a]:flex-col text-center",
            li {
                Link { to: Route::About {},
                    "⁉️"
                    span { "about" }
                }
            }
            li {
                Link { to: Route::ListOfDislikedProfiles {},
                    "💔"
                    span { "disliked" }
                }
            }
            li {
                Link { to: Route::ListOfUncheckedProfiles {},
                    "🎉"
                    span { "swipe" }
                }
            }
            li {
                Link { to: Route::ListOfLikedProfiles {},
                    "❤️"
                    span { "liked" }
                }
            }
            li {
                Link { to: Route::About {},
                    "🧑"
                    span { "profile" }
                }
            }
        }
    }
}
