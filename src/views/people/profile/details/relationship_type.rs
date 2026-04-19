use dioxus::prelude::*;

use crate::models::relationship_type::RelationshipType as ERT;
use crate::views::people::profile::details::DetailsCtx;

#[component]
pub(super) fn RelationshipType() -> Element {
    let mut dcx = use_context::<DetailsCtx>();

    let tmp = dcx.rw.read().relationship_type;
    let r = tmp.as_ref();
    let value = r.map(|rt| rt.to_string());

    rsx! {
        if (dcx.editing)() {
            div {
                select {
                    class: if value == None { "text-gray-500" },
                    value,
                    onchange: move |evt| {
                        dcx.rw.write().relationship_type = ERT::from_str(&evt.value());
                    },

                    option { value: "", "👩‍❤️‍👨 Your relationship type..." }
                    option { value: "{ERT::FiguringOutMyRelationshipType}",
                        "{ERT::FiguringOutMyRelationshipType}"
                    }
                    option { value: "{ERT::Monogamy}", "{ERT::Monogamy}" }
                    option { value: "{ERT::NonMonogamy}", "{ERT::NonMonogamy}" }
                }
            }
        } else {
            if let Some((emoji, text)) = r.map(|rt| rt.parts()) {
                div {
                    "{emoji}"
                    div { "{text}" }
                }
            }
        }
    }
}
