use dioxus::prelude::*;

use crate::models::person::{Frequency, Person};

#[component]
pub(super) fn Marijuana(sig: Signal<Person>, editing: bool) -> Element {
    let tmp = &sig.read().habits;
    let r = tmp.as_ref();
    let value = r
        .and_then(|h| h.marijuana.map(|d| d.to_string()))
        .unwrap_or_default();

    rsx! {
        if editing {
            li {
                "🌿🚬"

                select {
                    class: if value == "" { "text-gray-500" },
                    value,

                    onchange: move |evt| {
                        sig.with_mut(|p| {
                            let marijuana = Frequency::from_str(&evt.value());
                            if let Some(habits) = p.habits.as_mut() {
                                habits.marijuana = marijuana;
                            } else {
                                #[cfg(not(feature = "server"))]
                                {
                                    p.habits = Some(crate::models::person::Habits {
                                        marijuana,
                                        ..Default::default()
                                    });
                                }
                            }
                        });
                    },

                    option { value: "", "Marijuana?" }
                    option { value: "{Frequency::Never}", "{Frequency::Never}" }
                    option { value: "{Frequency::Rarely}", "{Frequency::Rarely}" }
                    option { value: "{Frequency::Often}", "{Frequency::Often}" }
                    option { value: "{Frequency::YesPlease}", "{Frequency::YesPlease}" }
                }
            }
        } else {
            if let Some(marijuana) = r.and_then(|h| h.marijuana.as_ref()) {
                li { title: "marijuana", "🌿🚬 {marijuana}" }
            }
        }
    }
}
