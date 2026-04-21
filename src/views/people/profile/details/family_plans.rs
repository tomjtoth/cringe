use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::models::FamilyPlans as EFP;
use crate::views::people::profile::details::DetailsCtx;
use crate::views::people::profile::ResourceCtx;

#[component]
pub(super) fn FamilyPlans() -> Element {
    let mut dcx = use_context::<DetailsCtx>();
    let rcx = use_context::<ResourceCtx>();

    let plans = &dcx.rw.read().family_plans;

    fn singular(e: &EFP) -> String {
        e.to_string()
            .replace("Doesn't", "I don't")
            .replace("Wants", "I want")
    }

    rsx! {
        if rcx.editing() {
            li {
                "🍼"
                select {
                    class: if plans.is_none() { "text-gray-500" },
                    onchange: move |evt| dcx.rw.with_mut(|p| p.family_plans = EFP::from_str(&evt.value())),

                    option { value: "", "Family plans?" }
                    for val in EFP::iter() {
                        option { value: "{val}", selected: *plans == Some(val),
                            "{singular(&val)}"
                            if val != EFP::NotSureYet {
                                if let Some(true) = dcx.rw.read().has_children {
                                    " more"
                                }
                                " children"
                            }
                        }
                    }

                }
            }
        } else {
            if let Some(wants) = &dcx.ro.read().family_plans {
                li {
                    "🍼 {wants}"
                    if *plans != Some(EFP::NotSureYet) {
                        if let Some(true) = dcx.ro.read().has_children {
                            " more"
                        }
                        " children"
                    }
                }
            }
        }
    }
}
