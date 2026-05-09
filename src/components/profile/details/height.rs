use dioxus::prelude::*;

use crate::components::profile::{ProfileCtx, ResourceCtx};
use crate::state::{TrMe, ME};

#[component]
pub(super) fn Height() -> Element {
    let pcx = use_context::<ProfileCtx>();
    let rcx = use_context::<ResourceCtx>();

    let onchange = use_callback(|s: String| {
        if let Ok(h) = s.parse::<u8>() {
            ME.mut_draft(|d| d.height = h);
        }
    });

    rsx! {
        if rcx.editing() {
            li {
                "📏"
                HeightInput {
                    value: ME.read().draft.as_ref().map(|d| d.height),
                    onchange,
                }
            }
        } else {
            li { "📏 {pcx.read().height} cm" }
        }
    }
}

#[component]
pub(in crate::components) fn HeightInput(value: Option<u8>, onchange: Callback<String>) -> Element {
    rsx! {
        input {
            required: true,
            r#type: "number",
            placeholder: "Height",
            class: "w-30",
            min: u8::MIN,
            max: u8::MAX,

            value,
            onchange: move |evt| onchange(evt.value()),
        }
    }
}
