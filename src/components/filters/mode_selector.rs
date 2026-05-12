use dioxus::prelude::*;

use crate::components::modal::{TrModal, MODALS};

#[component]
pub(super) fn ModeSelector() -> Element {
    let mut fcx = use_context::<super::FiltersCtx>();

    let strict = use_callback(move |_| {
        fcx.write().strict_mode = true;
    });

    let relaxed = use_callback(move |_| {
        fcx.write().strict_mode = false;
    });

    rsx! {
        button {
            onclick: move |evt| {
                evt.prevent_default();
                MODALS
                    .build("z-10")
                    .title("Filter mode")
                    .button("🙅 Strict", Some(strict))
                    .button("🤷 Relaxed", Some(relaxed))
                    .p(
                        "Strict mode excludes profiles with unset values for multi-select attributes you filter on, such as Relationship type, Zodiac signs and habits. Relaxed mode includes them.",
                    );
            },

            if fcx.read().strict_mode {
                "🙅 Strict"
            } else {
                "🤷 Relaxed"
            }
        }
    }
}
