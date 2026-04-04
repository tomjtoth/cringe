use dioxus::prelude::*;

use crate::{
    models::person::Decision,
    state::client::decide,
    views::people::{
        listing::ListingCtx,
        person::{PersonCtx, ResourceCtx},
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
    let pcx = use_context::<PersonCtx>();
    let olcx = use_context::<Option<ListingCtx>>();
    let rcx = use_context::<ResourceCtx>();

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

        // _dx_wants -> syntax highlight/complainer issue
        if let Some(ListingCtx { wants: _dx_wants, retainer }) = olcx {

            if decision != _dx_wants {
                if let Some(id) = (pcx.person)().id {
                    button {
                        class,

                        onclick: move |evt| async move {
                            evt.prevent_default();

                            if let Some(buttons_decision) = decision {
                                if let Ok(true) = decide(id, buttons_decision).await {
                                    retainer(id);
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
            button { class,
                if rcx.editing() {
                    "💾"
                } else {
                    "✏️"
                }
            }
        }
    }
}
