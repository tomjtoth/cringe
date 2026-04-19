use dioxus::prelude::*;

use crate::models::person::{Frequency, THabits};
use crate::views::people::profile::details::DetailsCtx;

#[component]
pub(super) fn Habit(
    children: Element,
    emoji: String,
    selector: fn(&THabits) -> Option<&Frequency>,
    onchange: fn(&mut THabits, Option<Frequency>),
) -> Element {
    let mut dcx = use_context::<DetailsCtx>();

    let tmp_ro = &dcx.ro.read().habits;
    let ro = tmp_ro.as_ref();

    let tmp_rw = &dcx.rw.read().habits;
    let rw = tmp_rw.as_ref();
    let value = rw
        .and_then(selector)
        .map(|d| d.to_string())
        .unwrap_or_default();

    rsx! {
        if (dcx.editing)() {
            li {
                "{emoji}"

                select {
                    class: if value == "" { "text-gray-500" },
                    value,

                    onchange: move |evt| {
                        dcx.rw
                            .with_mut(|p| {
                                let freq = Frequency::from_str(&evt.value());
                                // TODO: fallback could be implemented, but this is always populated as of 19.4
                                if let Some(h) = p.habits.as_mut() {
                                    onchange(h, freq);
                                }
                            });
                    },

                    {children}

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
