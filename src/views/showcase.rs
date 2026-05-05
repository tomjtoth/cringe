use dioxus::prelude::*;

use crate::components::{
    login::Login,
    modal::{TrModal, MODALS},
};

#[component]
pub fn Showcase() -> Element {
    let semver = env!("CARGO_PKG_VERSION");

    rsx! {

        h1 { class: "relative",
            "Cringe "

            sup { "{semver}" }

            button {
                class: "absolute right-2 text-lg",
                onclick: move |_| MODALS.new("z-10", true, rsx! {
                    Login {}
                }),

                "Login ➜]"
            }

        }

        p { class: "app-center text-center p-2",
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
