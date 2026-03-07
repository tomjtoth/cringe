use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        h1 { class: "capitalize", "disclaimer" }
        p { class: "p-2",
            b { "This app is invitation-only. There is no automated verification process. " }
            "Instead you invite your friends and moderate their photos. "
            "If your direct inviter deletes their account, their inviter will moderate your pics. "
        }
    }
}
