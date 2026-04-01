use dioxus::prelude::*;

#[cfg(not(feature = "server"))]
use crate::models::person::Habits as MHabits;

use crate::models::person::{Frequency, Person};

#[component]
pub(super) fn Habits(sig: Signal<Person>, editing: bool) -> Element {
    rsx! {
        if editing {
            li {
                "🍷"

                select {
                    class: if sig.read().habits.as_ref().map(|h| h.drinking).unwrap_or(None) == None { "text-gray-500" },
                    value: sig.read()
                        .habits
                        .as_ref()
                        .map(|h| h.drinking.as_ref().map(|d| d.to_string()).unwrap_or("".to_string()))
                        .unwrap_or("".to_string()),

                    onchange: move |evt| {
                        sig.with_mut(|p| {
                            let drinking = Frequency::from_str(&evt.value());
                            if let Some(habits) = p.habits.as_mut() {
                                habits.drinking = drinking;
                            } else {
                                #[cfg(not(feature = "server"))]
                                {
                                    p.habits = Some(MHabits {
                                        drinking,
                                        ..Default::default()
                                    });
                                }
                            }
                        });
                    },

                    option { value: "", "Alcohol?" }
                    option { value: "{Frequency::Never}", "{Frequency::Never}" }
                    option { value: "{Frequency::Rarely}", "{Frequency::Rarely}" }
                    option { value: "{Frequency::Often}", "{Frequency::Often}" }
                    option { value: "{Frequency::YesPlease}", "{Frequency::YesPlease}" }
                }
            }

            li {
                "🚬"

                select {
                    class: if sig.read().habits.as_ref().map(|h| h.smoking).unwrap_or(None) == None { "text-gray-500" },
                    value: sig.read()
                        .habits
                        .as_ref()
                        .map(|h| h.smoking.as_ref().map(|d| d.to_string()).unwrap_or("".to_string()))
                        .unwrap_or("".to_string()),

                    onchange: move |evt| {
                        sig.with_mut(|p| {
                            let smoking = Frequency::from_str(&evt.value());
                            if let Some(habits) = p.habits.as_mut() {
                                habits.smoking = smoking;
                            } else {
                                #[cfg(not(feature = "server"))]
                                {
                                    p.habits = Some(MHabits {
                                        smoking,
                                        ..Default::default()
                                    });
                                }
                            }
                        });
                    },

                    option { value: "", "Smokes?" }
                    option { value: "{Frequency::Never}", "{Frequency::Never}" }
                    option { value: "{Frequency::Rarely}", "{Frequency::Rarely}" }
                    option { value: "{Frequency::Often}", "{Frequency::Often}" }
                    option { value: "{Frequency::YesPlease}", "{Frequency::YesPlease}" }
                }
            }

            li {
                "🌿🚬"

                select {
                    class: if sig.read().habits.as_ref().map(|h| h.marijuana).unwrap_or(None) == None { "text-gray-500" },
                    value: sig.read()
                        .habits
                        .as_ref()
                        .map(|h| h.marijuana.as_ref().map(|d| d.to_string()).unwrap_or("".to_string()))
                        .unwrap_or("".to_string()),

                    onchange: move |evt| {
                        sig.with_mut(|p| {
                            let marijuana = Frequency::from_str(&evt.value());
                            if let Some(habits) = p.habits.as_mut() {
                                habits.marijuana = marijuana;
                            } else {
                                #[cfg(not(feature = "server"))]
                                {
                                    p.habits = Some(MHabits {
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

            li {
                "💊💉"

                select {
                    class: if sig.read().habits.as_ref().map(|h| h.drugs).unwrap_or(None) == None { "text-gray-500" },
                    value: sig.read()
                        .habits
                        .as_ref()
                        .map(|h| h.drugs.as_ref().map(|d| d.to_string()).unwrap_or("".to_string()))
                        .unwrap_or("".to_string()),

                    onchange: move |evt| {
                        sig.with_mut(|p| {
                            let drugs = Frequency::from_str(&evt.value());
                            if let Some(habits) = p.habits.as_mut() {
                                habits.drugs = drugs;
                            } else {
                                #[cfg(not(feature = "server"))]
                                {
                                    p.habits = Some(MHabits {
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
                if let Some(drinking) = habits.drinking {
                    li { "🍷 {drinking}" }
                }

                if let Some(smoking) = habits.smoking {
                    li { "🚬 {smoking}" }
                }

                if let Some(marijuana) = habits.marijuana {
                    li { title: "marijuana", "🌿🚬 {marijuana}" }
                }

                if let Some(drugs) = habits.drugs {
                    li { title: "drugs", "💊💉 {drugs}" }
                }
            }
        }
    }
}
