use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::models::FamilyPlans as EFP;

#[component]
pub(super) fn FamilyPlans() -> Element {
    let mut fcx = use_context::<super::FiltersCtx>();

    rsx! {
        div { class: "flex-col",
            "family plans:"

            div { class: "flex gap-2 text-nowrap",
                for val in EFP::iter() {
                    label {
                        input {
                            r#type: "checkbox",
                            name: "family_plans",
                            value: "{val}",
                            checked: fcx.read().family_plans.contains(&val),
                            onclick: move |_| {
                                fcx
                                    .with_mut(|ff| {
                                    if ff.family_plans.contains(&val) {
                                        ff.family_plans.retain(|f| f != &val);
                                    } else {
                                        ff.family_plans.push(val.clone());
                                    }
                                })
                            },
                        }
                        " {val}"
                    }
                }
            }
        }
    }
}
