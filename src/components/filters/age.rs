use dioxus::prelude::*;

#[component]
pub(super) fn Age() -> Element {
    let mut fcx = use_context::<super::FiltersCtx>();

    rsx! {
        div {
            "age:"

            input {
                class: "no-spinner",
                r#type: "number",
                placeholder: "min",
                min: 18,
                max: u16::MAX,
                value: fcx.read().age_min,
                onchange: move |evt| fcx.write().age_min = evt.value().parse().ok(),
            }

            "-"

            input {
                class: "no-spinner",
                r#type: "number",
                placeholder: "max",
                min: 18,
                max: u16::MAX,
                value: fcx.read().age_max,
                onchange: move |evt| fcx.write().age_max = evt.value().parse().ok(),
            }
        }
    }
}
