use dioxus::prelude::*;

use crate::{
    models::person::Decision,
    state::client::decide,
    views::people::{listing::ListingCtx, person::PersonCtx},
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
pub fn EditButton() -> Element {
    rsx! {
        Button {}
    }
}

#[component]
fn Button(decision: Option<Decision>) -> Element {
    let pc = use_context::<PersonCtx>();
    let olc = use_context::<Option<ListingCtx>>();

    let class = format!(
        "{} z-1 bottom-5 p-3! bg-background select-none",
        if decision == Some(Decision::Skip) {
            "sticky mt-2 left-5 rounded-full!"
        } else {
            "absolute right-5" // Like & Edit buttons
        }
    );

    rsx! {
        // we're on a listing, but the profile is Skipped and this is a Like button
        // _dx_wants -> syntax highlight/complainer issue
        if let Some(ListingCtx { wants: _dx_wants, retainer }) = olc {

            if decision != _dx_wants {
                if let Some(id) = pc.person.id {
                    button {
                        class,

                        onclick: move |_| async move {
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
            button { class, onclick: move |_| {}, "✏️" }
        }
    }
}
