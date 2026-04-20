use dioxus::prelude::*;

use crate::models::person::{Frequency, Habits as MHabits};
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
    selector: fn(&MHabits) -> Option<&Frequency>,
    onchange: fn(&mut MHabits, Option<Frequency>),
) -> Element {
    let rw_habits = &dcx.rw.read().habits;
    let rw_freq = selector(rw_habits);
    let value = rw_freq.map(|d| d.to_string()).unwrap_or_default();

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
                            onchange(&mut p.habits, freq);
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
            if let Some(frequency) = selector(&dcx.ro.read().habits) {
                li { "{emoji} {frequency}" }
            }
        }
    }
}
