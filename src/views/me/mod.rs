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
            if let Some(person) = ME().profile {
                div { class: "relative h-full overflow-y-scroll px-2",
                    Person { person }
                }
            } else {
                BasicMe {}
            }
        }
    }
}
