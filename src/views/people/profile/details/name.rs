use dioxus::prelude::*;

use crate::{
    state::ME,
    views::people::profile::{ProfileCtx, ResourceCtx},
};

#[component]
pub fn Name() -> Element {
    let pcx = use_context::<ProfileCtx>();
    let rcx = use_context::<ResourceCtx>();

    rsx! {
        if rcx.editing() {
            input {
                placeholder: "Your firstname",
                value: ME.read().draft.as_ref().map(|p| p.name.clone()),
                minlength: 2,
                onchange: move |evt| {
                    let name = evt.value();
                    if name.len() > 1 {
                        ME.with_mut(|me| me.draft.as_mut().unwrap().name = name);
                    }
                },
            }
        } else {
            span { class: "text-2xl", "{pcx.read().name}" }
        }

    }
}
