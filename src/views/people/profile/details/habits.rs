use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::models::{Frequency, Profile};
use crate::state::ME;
use crate::views::people::profile::{ProfileCtx, ResourceCtx};

#[component]
pub(super) fn Habits() -> Element {
    let pcx = use_context::<ProfileCtx>();
    let rcx = use_context::<ResourceCtx>();

    rsx! {
        {
            habit(
                "🍷",
                "Drinking",
                &pcx,
                &rcx,
                |h| h.drinking.as_ref(),
                |h, f| h.drinking = f,
            )
        }

        {
            habit(
                "🚬",
                "Smoking",
                &pcx,
                &rcx,
                |h| h.smoking.as_ref(),
                |h, f| h.smoking = f,
            )
        }

        {
            habit(
                "🌿🚬",
                "Marijuana",
                &pcx,
                &rcx,
                |h| h.marijuana.as_ref(),
                |h, f| h.marijuana = f,
            )
        }

        {habit("💊💉", "Drugs", &pcx, &rcx, |h| h.drugs.as_ref(), |h, f| h.drugs = f)}
    }
}

/// keeping this a simple fn due to selector and onchange
fn habit(
    emoji: &str,
    question: &str,
    pcx: &ProfileCtx,
    rcx: &ResourceCtx,
    selector: fn(&Profile) -> Option<&Frequency>,
    onchange: fn(&mut Profile, Option<Frequency>),
) -> Element {
    let tmp = ME.with(|me| me.draft.clone());
    let freq = tmp.as_deref().map(selector).flatten();

    rsx! {
        if rcx.editing() {
            li {
                "{emoji}"

                select {
                    class: if freq == None { "text-gray-500" },

                    onchange: move |evt| {
                        let v = evt.value();
                        ME.with_mut(|me| {
                            let freq = Frequency::from_str(&v);
                            onchange(me.draft.as_mut().unwrap(), freq);
                        })
                    },

                    option { value: "", "{question}?" }

                    for val in Frequency::iter() {
                        option { value: "{val}", selected: freq == Some(&val), "{val}" }
                    }
                }
            }
        } else {
            if let Some(frequency) = selector(&pcx.read()) {
                li { "{emoji} {frequency}" }
            }
        }
    }
}
