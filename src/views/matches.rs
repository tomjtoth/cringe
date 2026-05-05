use dioxus::prelude::*;

use crate::views::listing::ListingCtx;

#[component]
pub fn Matches() -> Element {
    let mut lcx = use_context::<ListingCtx>();
    use_effect(move || lcx.set(None));

    rsx! {
        div { class: "app-center",
            h1 { "TODO" }
        }
    }
}
