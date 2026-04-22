use dioxus::prelude::*;

use crate::{
    state::ME,
    views::{
        me::core::CoreData,
        people::{listing::ListingCtx, profile::Profile},
        protector::NeedsLogin,
    },
};

mod core;

#[component]
pub fn Me() -> Element {
    use_context_provider(|| None::<ListingCtx>);

    rsx! {
        NeedsLogin {
            if let Some(profile) = ME().profile {
                div { class: "relative h-full overflow-y-scroll px-2",
                    Profile { profile }
                }
            } else {
                CoreData {}
            }
        }
    }
}
