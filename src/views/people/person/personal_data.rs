use dioxus::prelude::*;

use crate::views::people::{
    listing::ListingCtx,
    person::{container::Container, PersonCtx, ResourceCtx},
};

#[component]
pub fn PersonalData() -> Element {
    let olcx = use_context::<Option<ListingCtx>>();
    let person = use_context::<PersonCtx>().person;
    let _rcx = use_context::<ResourceCtx>();

    let mut already_has_kids = false;

    rsx! {
        Container { class: "[&>*+*]:border-t [&>*+*]:p-2", wo_button: olcx.is_some(),
            ul { class: "p-2 flex overflow-x-scroll [&>*+*]:ml-2 [&>*+*]:border-l *:p-2 text-nowrap",

                if let Some(age) = person().age() {
                    li { "🎂 {age}" }
                }

                if let Some(dist) = person().distance() {
                    li { "{dist}" }
                }

                li { "{person().gender}" }

                li { "📏 {person().height} cm" }

                if let Some(city) = &person().location {
                    li { "📍 {city}" }
                }

                if let Some(kids) = &person().kids {
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

                if let Some(sign) = person().zodiac_sign() {
                    li { "{sign}" }
                }

                if let Some(habits) = &person().habits {
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

            if let Some(job) = &person().occupation {
                p { "💼 {job}" }
            }

            if let Some(edu) = &person().education {
                p { "🎓 {edu}" }
            }

            if let Some(city) = &person().hometown {
                p { "🏠 {city}" }
            }

            if let Some(seeking) = &person().seeking {
                p { "{seeking}" }
            }

            if let Some(relationship_type) = &person().relationship_type {
                p { "{relationship_type}" }
            }

        }

    }
}
