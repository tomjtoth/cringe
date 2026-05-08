use dioxus::prelude::*;

use crate::{
    components::{
        core_data::CoreData,
        login::Login,
        modal::{TrModal, MODALS},
        navbar::Navbar,
        router::Route,
    },
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
                Showcase { CoreData {} }
            }
        } else {
            Showcase {
                button {
                    class: "absolute top-2 right-2 text-lg",
                    onclick: move |_| MODALS.new("z-10", true, rsx! {
                        Login {}
                    }),

                    "Login ➜]"
                }

                div { class: "app-center text-center p-2",

                    p { class: "text-xl md:text-6xl xl:text-9xl mb-4",
                        "Cringe well"
                        s {
                            sup { "TM" }
                        }
                    }

                    p {
                        "This is a "
                        b { "Work-in-Progress" }
                        " Hinge clone. "
                        b { "Expect data loss" }
                        " below version 1.0.0! Check out the source code "
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
