use dioxus::prelude::*;

use crate::models::{
    person::Person, relationship_type::RelationshipType as ERT, seeking::Seeking as ES,
};
use crate::views::people::profile::details::DetailsCtx;

#[component]
pub(super) fn RelationshipType() -> Element {
    let dcx = use_context::<DetailsCtx>();

    helper(
        "👩‍❤️‍👨 Your relationship type...",
        &dcx,
        |p| p.relationship_type.as_ref(),
        |p, s| p.relationship_type = ERT::from_str(&s),
        |x| x.parts(),
        vec![
            ERT::FiguringOutMyRelationshipType,
            ERT::Monogamy,
            ERT::NonMonogamy,
        ],
    )
}

#[component]
pub(super) fn Seeking() -> Element {
    let dcx = use_context::<DetailsCtx>();

    helper(
        "🕵️ You're seeking...",
        &dcx,
        |p| p.seeking.as_ref(),
        |p, s| p.seeking = ES::from_str(&s),
        |x| x.parts(),
        vec![
            ES::LongTerm,
            ES::LongTermOpenToShort,
            ES::ShortTermFun,
            ES::ShortTermOpenToLong,
            ES::StillFiguringItOut,
        ],
    )
}

fn helper<T>(
    placeholder: &str,
    dcx: &DetailsCtx,
    selector: fn(&Person) -> Option<&T>,
    onchange: fn(&mut Person, String),
    parts_mapper: fn(&T) -> (&str, &str),
    map_these: Vec<T>,
) -> Element
where
    T: std::fmt::Display,
{
    let profile = dcx.rw.read();
    let as_ref = selector(&profile);
    let value = as_ref.map(|t| t.to_string());
    let mut rw = dcx.rw;

    rsx! {
        if (dcx.editing)() {
            div {
                select {
                    class: if value == None { "text-gray-500" },
                    value,
                    onchange: move |evt| onchange(&mut rw.write(), evt.value()),

                    option { value: "", "{placeholder}" }
                    for val in map_these {
                        option { value: "{val}", "{val}" }
                    }

                }
            }
        } else {
            if let Some((emoji, text)) = as_ref.map(parts_mapper) {
                div {
                    "{emoji}"
                    div { "{text}" }
                }
            }
        }
    }
}
