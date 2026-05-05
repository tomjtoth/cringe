use dioxus::prelude::*;

use crate::{
    state::ME,
    views::people::{listing::ListingCtx, profile::Profile},
};

pub mod core;

#[component]
pub fn Me() -> Element {
    use_context_provider(|| None::<ListingCtx>);

    rsx! {
        if let Some(profile) = ME().profile {
            div { class: "relative h-full overflow-y-scroll px-2",
                Profile { profile }
            }
        }
    }
}
