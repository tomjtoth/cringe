use dioxus::prelude::*;

use crate::views::people::profile::details::DetailsCtx;

#[component]
pub(super) fn HasChildren() -> Element {
    let mut dcx = use_context::<DetailsCtx>();

    let value = dcx.rw.read().has_children;

    rsx! {
        if (dcx.editing)() {
            li {
                "🧑‍🧒‍🧒"
                select {
                    class: if value.is_none() { "text-gray-500" },
                    value,
                    onchange: move |evt| dcx.rw.write().has_children = evt.value().parse::<bool>().ok(),

                    option { value: "", "Any children?" }
                    option { value: false, selected: value == Some(false), "I don't have children" }
                    option { value: true, selected: value == Some(true), "I have children" }
                }
            }
        } else {
            if let Some(has) = dcx.ro.read().has_children {
                li {
                    "🧑‍🧒‍🧒 Has"
                    if !has {
                        " no"
                    }
                    " children"
                }
            }
        }
    }
}
