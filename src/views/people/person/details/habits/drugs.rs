use dioxus::prelude::*;

use crate::models::person::{Frequency, Person};

#[component]
pub(super) fn Drugs(sig: Signal<Person>, editing: bool) -> Element {
    let value = sig
        .read()
        .habits
        .as_ref()
        .and_then(|h| h.drinking.map(|d| d.to_string()));

    rsx! {
        if editing {
            li {
                "💊💉"

                select {
                    class: if value == None { "text-gray-500" },
                    value,

                    onchange: move |evt| {
                        sig.with_mut(|p| {
                            let drugs = Frequency::from_str(&evt.value());
                            if let Some(habits) = p.habits.as_mut() {
                                habits.drugs = drugs;
                            } else {
                                #[cfg(not(feature = "server"))]
                                {
                                    p.habits = Some(crate::models::person::Habits {
                                        drugs,
                                        ..Default::default()
                                    });
                                }
                            }
                        });
                    },

                    option { value: "", "Drugs?" }
                    option { value: "{Frequency::Never}", "{Frequency::Never}" }
                    option { value: "{Frequency::Rarely}", "{Frequency::Rarely}" }
                    option { value: "{Frequency::Often}", "{Frequency::Often}" }
                    option { value: "{Frequency::YesPlease}", "{Frequency::YesPlease}" }
                }
            }
        } else {
            if let Some(habits) = sig.read().habits.as_ref() {
                if let Some(drugs) = habits.drugs {
                    li { title: "drugs", "💊💉 {drugs}" }
                }
            }
        }
    }
}
