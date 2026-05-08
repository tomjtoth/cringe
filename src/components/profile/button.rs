use dioxus::prelude::*;

use crate::{
    components::profile::{ProfileCtx, ResourceCtx},
    models::Decision,
    state::decide,
    views::listing::{ListingCtx, OTHERS},
};

#[component]
pub fn LikeButton() -> Element {
    rsx! {
        Button { decision: Decision::Like }
    }
}

#[component]
pub fn SkipButton() -> Element {
    rsx! {
        Button { decision: Decision::Skip }
    }
}

#[component]
fn Button(decision: Option<Decision>) -> Element {
    let pcx = use_context::<ProfileCtx>();
    let lcx = use_context::<ListingCtx>();
    let mut rcx = use_context::<ResourceCtx>();

    let class = format!(
        "{} z-1 absolute bottom-5 border-2! bg-background select-none",
        if decision == Some(Decision::Skip) {
            "left-5 rounded-full!"
        } else {
            "right-5" // Like & Edit buttons
        }
    );

    rsx! {
        // we're on a listing, but the profile is:
        // - either Skipped and this is a Like button
        // - or Liked and this is a Skip button
        if let Some(listing_wants) = lcx() {
            if decision != listing_wants {
                if let Some(id) = pcx.profile.read().id {
                    button {
                        class,

                        onclick: move |evt| async move {
                            evt.prevent_default();

                            if let Some(buttons_decision) = decision {
                                if let Ok(true) = decide(id, buttons_decision).await {
                                    OTHERS.write().retain(|p| p.id != Some(id));
                                }
                            }
                        },

                        if decision == Some(Decision::Skip) {
                            "🚫"
                        } else {
                            "✅"
                        }
                    }
                }
            }
        } else {
            button {
                class,
                onclick: move |evt| {
                    if !rcx.editing() {
                        evt.prevent_default();
                        rcx.toggle_editing()
                    }
                },

                if rcx.editing() {
                    "💾"
                } else {
                    "✏️"
                }
            }
        }
    }
}
