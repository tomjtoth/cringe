use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        h1 { class: "capitalize", "disclaimer" }

        p {
            "This is a "
            b { "Work-in-Progress & currently insecure" }
            " Hinge clone. "
        }

        p {
            b { "This app is invitation-only. There is no automated verification process. " }
            "Instead you invite your friends and moderate their photos. "
            "If your direct inviter deletes their account, their inviter will moderate your pics. "
        }
    }
}
