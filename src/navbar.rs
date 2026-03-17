use dioxus::prelude::*;

use crate::router::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { class: "grow overflow-hidden", Outlet::<Route> {} }

        ul { class: "pt-2 bg-background border-t flex justify-around w-full
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
