use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::models::Gender as EGender;
use crate::state::{TrMe, ME};
use crate::views::people::profile::{ProfileCtx, ResourceCtx};

#[component]
pub(super) fn Gender() -> Element {
    let pcx = use_context::<ProfileCtx>();
    let rcx = use_context::<ResourceCtx>();

    let value = ME.with(|me| me.draft.as_ref().and_then(|p| Some(p.gender)));

    let onchange = use_callback(|s: String| {
        ME.mut_draft(|d| {
            if let Some(g) = EGender::from_str(&s) {
                d.gender = g;

                if let Some(gi) = &d.gender_identity {
                    if !g.identities().contains(gi) {
                        d.gender_identity = None;
                    }
                }
            }
        });
    });

    rsx! {
        if rcx.editing() {
            li {
                GenderSelect { value, onchange }
            }
        } else {
            li { "{pcx.read().gender}" }
        }
    }
}

#[component]
pub(in crate::views) fn GenderSelect(
    value: Option<EGender>,
    onchange: Callback<String>,
) -> Element {
    rsx! {
        select { required: true, onchange: move |evt| onchange(evt.value()),

            option { value: "", disabled: true, "Your gender" }
            for gender in EGender::iter() {
                option { value: "{gender}", selected: value == Some(gender), "{gender}" }
            }
        }
    }
}
