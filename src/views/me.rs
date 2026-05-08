use dioxus::prelude::*;

use crate::{components::profile::Profile, state::ME, views::listing::LCX};

#[component]
pub fn Me() -> Element {
    use_effect(move || LCX.with_mut(|cx| *cx = None));

    rsx! {
        if let Some(profile) = ME().profile {
            div { class: "relative h-full overflow-y-scroll px-2",
                Profile { profile }
            }
        }
    }
}
