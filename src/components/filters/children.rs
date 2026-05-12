use dioxus::prelude::*;

#[component]
pub(super) fn Children() -> Element {
    let mut fcx = use_context::<super::FiltersCtx>();

    rsx! {
        div {
            "Children:"

            select {
                value: fcx.read().has_children,
                onchange: move |evt| fcx.write().has_children = evt.value().parse().ok(),

                option { value: "", selected: fcx.read().has_children == None, "May have" }
                option {
                    value: true,
                    selected: fcx.read().has_children == Some(true),
                    "Should have"
                }
                option {
                    value: false,
                    selected: fcx.read().has_children == Some(false),
                    "Should not have"
                }
            }
        }
    }
}
