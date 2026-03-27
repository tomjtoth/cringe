use dioxus::prelude::*;

use crate::{
    state::client::ME,
    views::{
        me::basic::BasicMe,
        people::{listing::ListingCtx, person::Person},
        protector::NeedsLogin,
    },
};

mod basic;

#[component]
pub fn Me() -> Element {
    use_context_provider(|| None::<ListingCtx>);

    rsx! {
        NeedsLogin {
            if let Some(Some(person)) = ME() {
                div { class: "relative h-full overflow-y-scroll",
                    Person { person }
                    a {
                        class: format!(
                            "{} {}",
                            "absolute left-1/2 -translate-x-1/2 bottom-2",
                            "border rounded p-2 cursor-pointer select-none",
                        ),
                        href: "/logout",
                        "logout"
                    }
                }
            } else {
                BasicMe {}
            }
        }
    }
}
