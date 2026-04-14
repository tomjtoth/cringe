use dioxus::prelude::*;

use crate::models::person::{Frequency, Person};

#[component]
pub(super) fn Drinking(sig: Signal<Person>, editing: bool) -> Element {
    let tmp = &sig.read().habits;
    let r = tmp.as_ref();
    let value = r
        .and_then(|h| h.drinking.map(|d| d.to_string()))
        .unwrap_or_default();

    rsx! {
        if editing {
            li {
                "🍷"

                select {
                    class: if value == "" { "text-gray-500" },
                    value,

                    onchange: move |evt| {
                        sig.with_mut(|p| {
                            let drinking = Frequency::from_str(&evt.value());
                            if let Some(habits) = p.habits.as_mut() {
                                habits.drinking = drinking;
                            } else {
                                #[cfg(not(feature = "server"))]
                                {
                                    p.habits = Some(crate::models::person::Habits {
                                        drinking,
                                        ..Default::default()
                                    });
                                }
                            }
                        });
                    },

                    option { value: "", "Drinking?" }
                    option { value: "{Frequency::Never}", "{Frequency::Never}" }
                    option { value: "{Frequency::Rarely}", "{Frequency::Rarely}" }
                    option { value: "{Frequency::Often}", "{Frequency::Often}" }
                    option { value: "{Frequency::YesPlease}", "{Frequency::YesPlease}" }
                }
            }
        } else {
            if let Some(drinking) = r.and_then(|h| h.drinking.as_ref()) {
                li { "🍷 {drinking}" }
            }
        }
    }
}
