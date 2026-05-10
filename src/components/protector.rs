use dioxus::prelude::*;

use crate::{
    components::{core_data::CoreData, navbar::Navbar, router::Route},
    state::ME,
    views::showcase::Showcase,
};

#[component]
pub fn Protector() -> Element {
    rsx! {
        if ME.read().authenticated {
            if ME.read().profile.is_some() {
                div { class: "grow overflow-hidden", Outlet::<Route> {} }
                Navbar {}
            } else {
                Showcase { hide_login: true, CoreData {} }
            }
        } else {
            Showcase {
                div { class: "app-center text-center",

                    p {
                        class: "text-nowrap text-3xl lg:text-6xl 2xl:text-9xl",
                        class: "mb-4 lg:mb-8 2xl:mb-12",

                        b { "Cringe much" }

                        sup { class: "text-[0.9em]",
                            s { "TM" }
                        }
                    }

                    p { "Just a Hinge clone with additional fictional characters." }

                    p {
                        "Check out the source code "
                        a {
                            class: "pre-preflight",
                            href: "https://github.com/tomjtoth/cringe",
                            target: "_blank",
                            "here"
                        }
                        "."
                    }
                }
            }
        }

    }
}
