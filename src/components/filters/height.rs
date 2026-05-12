use dioxus::prelude::*;

#[component]
pub(super) fn Height() -> Element {
    let mut fcx = use_context::<super::FiltersCtx>();

    rsx! {
        div {
            "Height:"

            input {
                r#type: "number",
                placeholder: "min",
                min: u8::MIN,
                max: u8::MAX,
                value: fcx.read().height_min,
                oninput: move |evt| fcx.write().height_min = evt.value().parse().ok(),
            }

            "-"

            input {
                r#type: "number",
                placeholder: "max",
                min: u8::MIN,
                max: u8::MAX,
                value: fcx.read().height_max,
                onchange: move |evt| fcx.write().height_max = evt.value().parse().ok(),
            }
        }
    }
}
