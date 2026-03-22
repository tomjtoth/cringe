use dioxus::prelude::*;

use crate::{
    state::client::ME,
    views::{me::basic::BasicMe, people::person::Person, protector::NeedsLogin},
};

mod basic;

#[component]
pub fn Me() -> Element {
    rsx! {
        NeedsLogin {
            if let Some(Some(person)) = ME() {
                div { class: "h-full overflow-y-scroll",
                    Person { person, wo_buttons: true }
                }
            } else {
                BasicMe {}
            }
        }
    }
}
