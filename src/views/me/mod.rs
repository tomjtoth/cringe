use dioxus::prelude::*;

use crate::{state::client::ME, views::people::person::Person};

#[component]
pub fn Me() -> Element {
    rsx! {
        if let Some(authorized) = ME() {
            if let Some(profile) = authorized {
                div { class: "h-full overflow-y-scroll",
                    Person { person: profile.clone(), wo_buttons: true }
                }
            } else {
                p { "You don't have a profile!" }
            }
        } else {
            p { "You are not even logged in!" }
        }
    }
}
