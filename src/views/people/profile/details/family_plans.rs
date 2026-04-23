use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::models::FamilyPlans as EFP;
use crate::state::{TrMe, ME};
use crate::views::people::profile::{ProfileCtx, ResourceCtx};

#[component]
pub(super) fn FamilyPlans() -> Element {
    let pcx = use_context::<ProfileCtx>();
    let rcx = use_context::<ResourceCtx>();

    let plans = ME.with(|me| me.draft.as_ref().and_then(|p| p.family_plans.clone()));

    let has_children = ME.with(|me| me.draft.as_ref().and_then(|p| p.has_children));

    fn first_person(e: &EFP) -> String {
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
                    onchange: move |evt| ME.mut_draft(|d| d.family_plans = EFP::from_str(&evt.value())),

                    option { value: "", "Family plans?" }
                    for val in EFP::iter() {
                        option { value: "{val}", selected: plans == Some(val),
                            "{first_person(&val)}"
                            if val != EFP::NotSureYet {
                                if let Some(true) = has_children {
                                    " more"
                                }
                                " children"
                            }
                        }
                    }

                }
            }
        } else {
            if let Some(wants) = &pcx.read().family_plans {
                li {
                    "🍼 {wants}"
                    if *wants != EFP::NotSureYet {
                        if let Some(true) = pcx.read().has_children {
                            " more"
                        }
                        " children"
                    }
                }
            }
        }
    }
}
