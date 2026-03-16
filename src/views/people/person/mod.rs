use dioxus::prelude::*;

use crate::{
    models::person::Gender, state::client::PEEPS, views::people::person::container::Container,
};

mod buttons;
mod container;
mod img;
mod prompt;

use img::Image;
use prompt::Prompt;

#[component]
pub fn Person(id: i32) -> Element {
    let person = PEEPS.iter().find(|p| p.id == Some(id)).unwrap();

    let mut already_has_kids = false;

    let mut pics = person.pictures.clone().into_iter();
    let mut prompts = person.prompts.clone().into_iter();

    rsx! {

        div { class: "relative md:columns-3 *:mb-2 text-lg",
            h2 { "{person.name}" }

            Image { id, src: pics.next() }
            Prompt { id, prompt: prompts.next() }

            Container { class: "[&>*+*]:border-t [&>*+*]:p-2",
                ul { class: "p-2 flex overflow-x-scroll [&>*+*]:ml-2 [&>*+*]:border-l *:p-2 text-nowrap",

                    li { "🎂 {person.age()}" }

                    li {
                        if matches!(person.gender, Gender::Male) {
                            "♂️ Man"
                        } else {
                            "♀️ Woman"
                        }
                    }

                    li { "📏 {person.height} cm" }

                    if let Some(city) = &person.location.city {
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
                                } else {
                                    "Doesn't want"
                                }
                                if already_has_kids {
                                    " more"
                                }
                                " kids"
                            }
                        }
                    }

                    li { "{person.zodiac_sign()}" }

                    if let Some(drinking) = &person.drinking {
                        li { "🍷 {drinking}" }
                    }

                    if let Some(smoking) = &person.smoking {
                        li { "🚬 {smoking}" }
                    }

                    if let Some(marijuana) = &person.marijuana {
                        li { title: "marijuana", "🚬🥦 {marijuana}" }
                    }

                    if let Some(drugs) = &person.drugs {
                        li { title: "drugs", "💊💉 {drugs}" }
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
            }

            Image { id, src: pics.next() }
            Prompt { id, prompt: prompts.next() }

            Image { id, src: pics.next() }
            Prompt { id, prompt: prompts.next() }

            Image { id, src: pics.next() }
            Prompt { id, prompt: prompts.next() }

            Image { id, src: pics.next() }
            Prompt { id, prompt: prompts.next() }

            Image { id, src: pics.next() }
            Prompt { id, prompt: prompts.next() }

            buttons::DislikeButton { id }
        }

    }
}
