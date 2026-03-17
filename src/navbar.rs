use dioxus::prelude::*;

use crate::router::Route;
use crate::state::client::get_my_pic;

#[component]
pub fn Navbar() -> Element {
    let mut avatar_url = use_signal(|| None::<String>);

    let _ = use_server_future(move || async move {
        if let Ok(Some(pic)) = get_my_pic().await {
            avatar_url.set(Some(pic.src()));
        }
    });

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

                    if let Some(src) = avatar_url() {
                        img { class: "w-6 border rounded-full", src }
                    } else {
                        "🧑"
                        span { "profile" }
                    }
                }
            }
        }
    }
}
