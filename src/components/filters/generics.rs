use dioxus::prelude::*;
use strum::{EnumProperty, IntoEnumIterator};

use crate::models::{
    FamilyPlans as EFP, Frequency, GenderIdentity as EGI, RelationshipType as ERT, Seeking as ES,
    ZodiacSign as EZS,
};

macro_rules! multi_rsx {
    ($enum:ty, $vec:ident, $fcx:expr, $class:expr) => {
        rsx! {
            div { class: $class,
                for (val, checked, glow) in <$enum>::iter()
                    .map(|e| (
                        e,
                        $fcx.read().$vec.contains(&e),
                        e.get_str("glow")
                    ))
                {
                    label {
                        input {
                            r#type: "checkbox",
                            name: stringify!($vec),
                            value: "{val}",
                            checked,

                            onclick: move |_| {
                                $fcx.with_mut(|ff| {
                                    if ff.$vec.contains(&val) {
                                        ff.$vec.retain(|f| f != &val);
                                    } else {
                                        ff.$vec.push(val.clone());
                                    }
                                });
                            },
                        }

                        if let (Some((ico, txt)), Some(glow)) =
                            (val.to_string().split_once(" "), glow)
                        {
                            span {
                                class: if checked {
                                    "text-shadow-[0_0_1px,0_0_2px,0_0_3px,0_0_4px,0_0_5px] {glow}"
                                },

                                {ico}
                            }

                            span {
                                class: if checked {
                                    "text-shadow-[0_0_2px]"
                                } else {
                                    "text-gray-500"
                                },

                                " {txt}"
                            }
                        } else {
                            span {
                                class: if checked {
                                    "text-shadow-[0_0_2px]"
                                } else {
                                    "text-gray-500"
                                },

                                "{val}"
                            }
                        }
                    }
                }
            }
        }
    };

    ($enum:ty, $vec:ident, $fcx:expr) => {
        multi_rsx!($enum, $vec, $fcx, "flex-col")
    };
}

#[component]
pub(super) fn Habits() -> Element {
    let mut fcx = use_context::<super::FiltersCtx>();

    rsx! {
        div { class: "flex-wrap justify-around [&_div]:flex-col",
            div {
                span { "🍷 Drinking" }
                {multi_rsx!(Frequency, drinking, fcx)}
            }

            div {
                span { "🚬 Smoking" }
                {multi_rsx!(Frequency, smoking, fcx)}
            }

            div {
                span { "🌿🚬 Marijuana" }
                {multi_rsx!(Frequency, marijuana, fcx)}
            }

            div {
                span { "💊💉 Drugs" }
                {multi_rsx!(Frequency, drugs, fcx)}
            }
        }
    }
}

#[component]
pub(super) fn Seeking() -> Element {
    let mut fcx = use_context::<super::FiltersCtx>();
    multi_rsx!(ES, seeking, fcx)
}

#[component]
pub(super) fn RelationshipType() -> Element {
    let mut fcx = use_context::<super::FiltersCtx>();
    multi_rsx!(ERT, relationship_type, fcx)
}

#[component]
pub(super) fn ZodiacSign() -> Element {
    let mut fcx = use_context::<super::FiltersCtx>();
    multi_rsx!(
        EZS,
        zodiac_sign,
        fcx,
        "flex-wrap justify-around lg:max-w-135"
    )
}

#[component]
pub(super) fn FamilyPlans() -> Element {
    let mut fcx = use_context::<super::FiltersCtx>();
    multi_rsx!(EFP, family_plans, fcx, "self-stretch justify-around")
}

#[component]
pub(super) fn GenderIdentity() -> Element {
    let mut fcx = use_context::<super::FiltersCtx>();
    multi_rsx!(
        EGI,
        gender_identity,
        fcx,
        // TODO: trying to match the width of habits
        "flex-wrap justify-around lg:max-w-135"
    )
}
