use dioxus::prelude::*;

use crate::models::person::{Frequency, THabits};
use crate::views::people::profile::details::DetailsCtx;

#[component]
pub(super) fn Habits() -> Element {
    let dcx = use_context::<DetailsCtx>();

    rsx! {
        {habit("🍷", "Drinking", &dcx, |h| h.drinking.as_ref(), |h, f| h.drinking = f)}

        {habit("🚬", "Smoking", &dcx, |h| h.smoking.as_ref(), |h, f| h.smoking = f)}

        {
            habit(
                "🌿🚬",
                "Marijuana",
                &dcx,
                |h| h.marijuana.as_ref(),
                |h, f| h.marijuana = f,
            )
        }

        {habit("💊💉", "Drugs", &dcx, |h| h.drugs.as_ref(), |h, f| h.drugs = f)}
    }
}

/// keeping this a simple fn due to selector and onchange
fn habit(
    emoji: &str,
    question: &str,
    dcx: &DetailsCtx,
    selector: fn(&THabits) -> Option<&Frequency>,
    onchange: fn(&mut THabits, Option<Frequency>),
) -> Element {
    let tmp_ro = &dcx.ro.read().habits;
    let ro = tmp_ro.as_ref();

    let tmp_rw = &dcx.rw.read().habits;
    let rw = tmp_rw.as_ref();
    let value = rw
        .and_then(selector)
        .map(|d| d.to_string())
        .unwrap_or_default();

    let mut rw = dcx.rw;

    rsx! {
        if (dcx.editing)() {
            li {
                "{emoji}"

                select {
                    class: if value == "" { "text-gray-500" },
                    value,

                    onchange: move |evt| {
                        rw
                            .with_mut(|p| {
                            let freq = Frequency::from_str(&evt.value());
                            // TODO: fallback could be implemented, but this is always populated as of 19.4
                            if let Some(h) = p.habits.as_mut() {
                                onchange(h, freq);
                            }
                        });
                    },

                    option { value: "", "{question}?" }
                    option { value: "{Frequency::Never}", "{Frequency::Never}" }
                    option { value: "{Frequency::Rarely}", "{Frequency::Rarely}" }
                    option { value: "{Frequency::Often}", "{Frequency::Often}" }
                    option { value: "{Frequency::YesPlease}", "{Frequency::YesPlease}" }
                }
            }
        } else {
            if let Some(frequency) = ro.and_then(selector) {
                li { "{emoji} {frequency}" }
            }
        }
    }
}
