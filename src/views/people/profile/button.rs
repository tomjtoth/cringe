use dioxus::prelude::*;

use crate::{
    models::person::Decision,
    state::decide,
    views::people::{
        listing::{ListingCtx, OTHERS},
        profile::{ProfileCtx, ResourceCtx},
    },
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
    let olcx = use_context::<Option<ListingCtx>>();
    let mut rcx = use_context::<ResourceCtx>();

    let class = format!(
        "{} z-1 bottom-5 border-2! bg-background select-none",
        if decision == Some(Decision::Skip) {
            "sticky mt-2 left-5 rounded-full!"
        } else {
            "absolute right-5" // Like & Edit buttons
        }
    );

    rsx! {
        // we're on a listing, but the profile is:
        // - either Skipped and this is a Like button
        // - or Liked and this is a Skip button
        if let Some(listing_wants) = olcx {
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
