use dioxus::prelude::*;

use crate::models::seeking::Seeking as ES;
use crate::views::people::profile::details::DetailsCtx;

#[component]
pub(super) fn Seeking() -> Element {
    let mut dcx = use_context::<DetailsCtx>();

    let tmp = dcx.rw.read().seeking;
    let r = tmp.as_ref();
    let value = r.map(|s| s.to_string());

    rsx! {
        if (dcx.editing)() {
            div {
                select {
                    class: if value == None { "text-gray-500" },
                    value,
                    onchange: move |evt| {
                        dcx.rw.write().seeking = ES::from_str(&evt.value());
                    },

                    option { value: "", class: "text-gray-500", "🕵️ You're seeking..." }
                    option { value: "{ES::LongTerm}", "{ES::LongTerm}" }
                    option { value: "{ES::LongTermOpenToShort}", "{ES::LongTermOpenToShort}" }
                    option { value: "{ES::ShortTermFun}", "{ES::ShortTermFun}" }
                    option { value: "{ES::ShortTermOpenToLong}", "{ES::ShortTermOpenToLong}" }
                    option { value: "{ES::StillFiguringItOut}", "{ES::StillFiguringItOut}" }
                }
            }
        } else {
            if let Some((emoji, text)) = r.map(|s| s.parts()) {
                div {
                    "{emoji}"
                    div { "{text}" }
                }
            }
        }
    }
}
