use dioxus::prelude::*;

#[component]
pub(super) fn ImageCount() -> Element {
    let mut fcx = use_context::<super::FiltersCtx>();

    rsx! {
        div {
            "images:"

            input {
                class: "no-spinner",
                r#type: "number",
                placeholder: "min",
                min: u8::MIN,
                max: 6,
                value: fcx.read().image_count_min,
                onchange: move |evt| fcx.write().image_count_min = evt.value().parse().ok(),
            }
        }
    }
}
