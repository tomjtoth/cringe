use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::models::family_plans::FamilyPlans as EFP;
use crate::views::people::profile::details::DetailsCtx;

#[component]
pub(super) fn FamilyPlans() -> Element {
    let mut dcx = use_context::<DetailsCtx>();

    let plans = &dcx.rw.read().family_plans;
    let value = plans.as_ref().map(|e| e.to_string());

    rsx! {
        if (dcx.editing)() {
            li {
                "🍼"
                select {
                    class: if value.is_none() { "text-gray-500" },
                    value,
                    onchange: move |evt| dcx.rw.with_mut(|p| p.family_plans = EFP::from_str(&evt.value())),

                    option { value: "", "Family plans?" }
                    for val in EFP::iter() {
                        option { value: "{val}", selected: *plans == Some(val), "{val}" }
                    }

                }
            }
        } else {
            if let Some(wants) = &dcx.ro.read().family_plans {
                li { "🍼 {wants}" }
            }
        }
    }
}
