use dioxus::prelude::*;

use crate::{
    components::profile::ResourceCtx,
    state::{TrMe, ME},
};

#[component]
pub fn Name() -> Element {
    let rcx = use_context::<ResourceCtx>();

    rsx! {
        if rcx.editing() {
            div {
                "🧑"

                input {
                    placeholder: "Your firstname",
                    value: ME.read().draft.as_ref().map(|p| p.name.clone()),
                    required: true,
                    minlength: 2,
                    onchange: move |evt| {
                        ME
                            .mut_draft(|d| {
                            let name = evt.value();
                            if name.len() > 1 {
                                d.name = name;
                            }
                        })
                    },
                }
            }
        }
    }
}

#[component]
pub fn NameInput(value: String, onchange: Callback<String>) -> Element {
    rsx! {
        input {
            placeholder: "Your firstname",
            class: "placeholder:text-center w-40 text-center",
            minlength: 2,
            required: true,

            value,
            onchange: move |evt| onchange(evt.value()),
        }
    }
}
