use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    let semver = env!("CARGO_PKG_VERSION");

    rsx! {
        h1 {
            "Cringe "

            sup { "{semver}" }
        }

        p { class: "p-2",
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
