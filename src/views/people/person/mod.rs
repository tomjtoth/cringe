use dioxus::prelude::*;

use crate::{
    models::person::Person as MPerson,
    views::people::{
        listing::ListingCtx,
        person::{button::SkipButton, personal_data::PersonalData},
    },
};

mod button;
mod container;
mod img;
mod personal_data;
mod prompt;

use img::Image;
use prompt::Prompt;

#[derive(Clone)]
struct PersonCtx {
    person: Signal<MPerson>,
}

#[derive(Clone)]
struct ResourceCtx {
    state: Signal<u8>,
}

impl ResourceCtx {
    fn provide() -> Self {
        let state = use_signal(|| 0);
        use_context_provider(|| ResourceCtx { state })
    }

    fn editing(&self) -> bool {
        (self.state)() >= 1
    }

    fn next_state(&mut self) {
        let curr = (self.state)();
        *(self.state).write() = if curr == 2 { 0 } else { curr + 1 };
    }

    fn submitting(&self) -> bool {
        (self.state)() == 2
    }
}

#[component]
pub fn Person(person: MPerson, editing: Option<bool>) -> Element {
    let ctx = use_context_provider(move || PersonCtx { person });

    let person = ctx.person;

    let mut already_has_kids = false;

    let mut pics = person.pics().clone().into_iter();
    let mut prompts = person.prompts().clone().into_iter();

    rsx! {
        div { class: "relative md:columns-2 lg:columns-3 *:mb-2 text-lg",
            h2 { class: "m-0! p-2 sticky z-2 top-0 bg-background ", "{person.name}" }

            Image { src: pics.next() }
            Prompt { prompt: prompts.next() }

            Container { class: "[&>*+*]:border-t [&>*+*]:p-2", wo_button: true,
                ul { class: "p-2 flex overflow-x-scroll [&>*+*]:ml-2 [&>*+*]:border-l *:p-2 text-nowrap",

                    if let Some(age) = person.age() {
                        li { "🎂 {age}" }
                    }

                    if let Some(dist) = person.distance() {
                        li { "{dist}" }
                    }

                    li { "{person.gender}" }

                    li { "📏 {person.height} cm" }

                    if let Some(city) = &person.location {
                        li { "📍 {city}" }
                    }

                    if let Some(kids) = &person.kids {
                        if let Some(has) = kids.has {
                            li {
                                "🧑‍🧒‍🧒 "
                                if has > 0 {
                                    "Has {has}"
                                    {already_has_kids = true}
                                } else {
                                    "No"
                                }
                                " kids"
                            }
                        }

                        if let Some(wants) = kids.wants {
                            li {
                                "🍼 "
                                if wants > 0 {
                                    "Wants {wants}"
                                } else if wants == 0 {
                                    "Doesn't want"
                                } else {
                                    "Doesn't know if wants any"
                                }
                                if already_has_kids {
                                    " more"
                                }
                                " kids"
                            }
                        }
                    }

                    if let Some(sign) = person.zodiac_sign() {
                        li { "{sign}" }
                    }

                    if let Some(habits) = &person.habits {
                        if let Some(drinking) = habits.drinking {
                            li { "🍷 {drinking}" }
                        }

                        if let Some(smoking) = habits.smoking {
                            li { "🚬 {smoking}" }
                        }

                        if let Some(marijuana) = habits.marijuana {
                            li { title: "marijuana", "🚬🥦 {marijuana}" }
                        }

                        if let Some(drugs) = habits.drugs {
                            li { title: "drugs", "💊💉 {drugs}" }
                        }
                    }
                }

                if let Some(job) = &person.occupation {
                    p { "💼 {job}" }
                }

                if let Some(edu) = &person.education {
                    p { "🎓 {edu}" }
                }

                if let Some(city) = &person.hometown {
                    p { "🏠 {city}" }
                }

                if let Some(seeking) = &person.seeking {
                    p { "{seeking}" }
                }

                if let Some(relationship_type) = &person.relationship_type {
                    p { "{relationship_type}" }
                }

            }

            Image { src: pics.next() }
            Prompt { prompt: prompts.next() }

            Image { src: pics.next() }
            Prompt { prompt: prompts.next() }

            Image { src: pics.next() }
            Prompt { prompt: prompts.next() }

            Image { src: pics.next() }
            Prompt { prompt: prompts.next() }

            Image { src: pics.next() }
            Prompt { prompt: prompts.next() }

            SkipButton {}
        }

    }
}
