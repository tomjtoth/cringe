use dioxus::prelude::*;

#[component]
pub(super) fn Distance() -> Element {
    let mut fcx = use_context::<super::FiltersCtx>();

    rsx! {

        div {
            "distance [km]:"

            input {
                class: "no-spinner",
                r#type: "number",
                placeholder: "max",
                min: u8::MIN,
                max: u8::MAX,
                value: fcx.read().distance,
                onchange: move |evt| fcx.write().distance = evt.value().parse().ok(),
            }
        }
    }
}
