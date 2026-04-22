use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::models::Gender as EGender;
use crate::state::ME;
use crate::views::people::profile::{ProfileCtx, ResourceCtx};

#[component]
pub(super) fn Gender() -> Element {
    let pcx = use_context::<ProfileCtx>();
    let rcx = use_context::<ResourceCtx>();

    let value = ME.with(|me| me.draft.as_ref().and_then(|p| Some(p.gender)));

    rsx! {
        if rcx.editing() {
            li {
                select {
                    onchange: move |evt| {
                        ME.with_mut(|me| {
                            if let Some(g) = EGender::from_str(&evt.value()) {
                                let me = me.draft.as_mut().unwrap();
                                me.gender = g;

                                if let Some(gi) = &me.gender_identity {
                                    if !g.identities().contains(gi) {
                                        me.gender_identity = None;
                                    }
                                }

                            }
                        })
                    },

                    for val in EGender::iter() {
                        option { value: "{val}", selected: value == Some(val), "{val}" }
                    }

                }
            }
        } else {
            li { "{pcx.read().gender}" }
        }
    }
}
