use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::models::{Person, RelationshipType as ERT, Seeking as ES};
use crate::views::people::profile::details::DetailsCtx;
use crate::views::people::profile::ResourceCtx;

#[component]
pub(super) fn RelationshipType() -> Element {
    helper_wo_cx(
        "👩‍❤️‍👨 Your relationship type...",
        |p| p.relationship_type.as_ref(),
        |p, s| p.relationship_type = ERT::from_str(&s),
        |x| x.parts(),
        ERT::iter().collect(),
    )
}

#[component]
pub(super) fn Seeking() -> Element {
    helper_wo_cx(
        "🕵️ You're seeking...",
        |p| p.seeking.as_ref(),
        |p, s| p.seeking = ES::from_str(&s),
        |x| x.parts(),
        ES::iter().collect(),
    )
}

fn helper_wo_cx<T>(
    placeholder: &str,
    selector: fn(&Person) -> Option<&T>,
    onchange: fn(&mut Person, String),
    parts_mapper: fn(&T) -> (&str, &str),
    map_these: Vec<T>,
) -> Element
where
    T: std::fmt::Display + PartialEq,
{
    let dcx = use_context::<DetailsCtx>();
    let rcx = use_context::<ResourceCtx>();

    helper(
        placeholder,
        &dcx,
        &rcx,
        selector,
        onchange,
        parts_mapper,
        map_these,
    )
}

fn helper<T>(
    placeholder: &str,
    dcx: &DetailsCtx,
    rcx: &ResourceCtx,
    selector: fn(&Person) -> Option<&T>,
    onchange: fn(&mut Person, String),
    parts_mapper: fn(&T) -> (&str, &str),
    map_these: Vec<T>,
) -> Element
where
    T: std::fmt::Display + PartialEq,
{
    let profile = dcx.rw.read();
    let as_ref = selector(&profile);
    let value = as_ref.map(|t| t.to_string());
    let mut rw = dcx.rw;

    rsx! {
        if rcx.editing() {
            div {
                select {
                    class: if value == None { "text-gray-500" },
                    value,
                    onchange: move |evt| onchange(&mut rw.write(), evt.value()),

                    option { value: "", "{placeholder}" }
                    for val in map_these {
                        option { value: "{val}", selected: as_ref == Some(&val), "{val}" }
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
