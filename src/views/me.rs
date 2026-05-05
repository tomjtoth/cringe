use dioxus::prelude::*;

use crate::{components::profile::Profile, state::ME, views::listing::ListingCtx};

#[component]
pub fn Me() -> Element {
    let mut lcx = use_context::<ListingCtx>();
    use_effect(move || lcx.set(None));

    rsx! {
        if let Some(profile) = ME().profile {
            div { class: "relative h-full overflow-y-scroll px-2",
                Profile { profile }
            }
        }
    }
}
