use dioxus::prelude::*;

#[component]
pub(super) fn ImageCount() -> Element {
    let mut fcx = use_context::<super::FiltersCtx>();

    rsx! {
        div {
            "Image count:"

            input {
                r#type: "number",
                placeholder: "min #",
                min: 0,
                max: 6,
                value: fcx.read().image_count_min,
                oninput: move |evt| {
                    fcx.write().image_count_min = evt.value().parse().ok().filter(|n| *n <= 6);
                },
            }
        }
    }
}
