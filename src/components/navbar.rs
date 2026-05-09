use dioxus::prelude::*;

use crate::{
    components::{
        filters::Filters,
        modal::{TrModal, MODALS},
        router::Route,
    },
    state::ME,
};

#[component]
pub fn Navbar() -> Element {
    let me = ME.read();
    let route: Route = use_route();
    let on_listing = route == Route::Listing {};

    rsx! {
        ul {
            class: "py-2 bg-background border-t flex items-center justify-around w-full *:select-none",
            class: "[&>li>*]:flex [&>li>*]:flex-col [&>li>*]:items-center text-center text-lg",

            li {
                if on_listing {
                    div {
                        class: "border-none! cursor-pointer",
                        onclick: move |_| {
                            MODALS.new("z-5", true, Filters());
                        },

                        span {
                            sub { class: "text-xs", "🚫" }

                            "⚙️"

                            sub { class: "text-xs", "✅" }
                        }
                        span { "filters" }
                    }
                } else {
                    Link { to: Route::Listing {},
                        span {
                            sub { class: "text-xs", "🚫" }

                            "😬"

                            sub { class: "text-xs", "✅" }
                        }
                        span { "cringe" }
                    }
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
                    if let Some(me) = me.profile.as_ref() {
                        if let Some(image) = me.images.get(0) {
                            img {
                                class: "w-6 border rounded-full",
                                src: image.src(),
                            }
                        } else {
                            "🧑"
                        }
                        span { "{me.name}" }
                    } else {
                        "🧑"
                        span { "profile" }
                    }
                }
            }
        }
    }
}
