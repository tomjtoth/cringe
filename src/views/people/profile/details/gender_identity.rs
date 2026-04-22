use dioxus::prelude::*;

use crate::models::GenderIdentity as GI;
use crate::state::ME;
use crate::views::people::profile::ProfileCtx;
use crate::views::people::profile::ResourceCtx;

#[component]
pub(super) fn GenderIdentity() -> Element {
    let pcx = use_context::<ProfileCtx>();
    let rcx = use_context::<ResourceCtx>();

    let gender = ME.with(|me| me.draft.as_ref().and_then(|p| Some(p.gender)));
    let gi = ME.with(|me| me.draft.as_ref().and_then(|p| p.gender_identity.clone()));

    rsx! {
        if rcx.editing() {
            li {
                "🏳️‍🌈"
                select {
                    class: if gi.is_none() { "text-gray-500" },
                    onchange: move |evt| {
                        ME.with_mut(|me| {
                            me.draft.as_mut().unwrap().gender_identity = GI::from_str(&evt.value());
                        })
                    },

                    option { value: "", "You identify as.." }
                    if let Some(g) = gender {
                        for val in &g.identities() {
                            option {
                                key: "{val}",
                                value: "{val}",
                                selected: gi.as_ref().filter(|gi| { *gi == val }).is_some(),
                                "{val}"
                            }
                        }
                    }

                }
            }
        } else {
            if let Some(identity) = &pcx.read().gender_identity {
                li { "🏳️‍🌈 {identity}" }
            }
        }
    }
}
