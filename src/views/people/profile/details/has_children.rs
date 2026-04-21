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

                    option { value: "", "Any kids?" }
                    option { value: false, "I don't have kids" }
                    option { value: true, "I have kids" }
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
