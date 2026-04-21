use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::models::{Profile, RelationshipType as ERT, Seeking as ES};
use crate::state::ME;
use crate::views::people::profile::{ProfileCtx, ResourceCtx};

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
    selector: fn(&Profile) -> Option<&T>,
    onchange: fn(&mut Profile, String),
    parts_mapper: fn(&T) -> (&str, &str),
    map_these: Vec<T>,
) -> Element
where
    T: std::fmt::Display + PartialEq,
{
    let pcx = use_context::<ProfileCtx>();
    let rcx = use_context::<ResourceCtx>();

    helper(
        placeholder,
        &pcx,
        &rcx,
        selector,
        onchange,
        parts_mapper,
        map_these,
    )
}

fn helper<T>(
    placeholder: &str,
    pcx: &ProfileCtx,
    rcx: &ResourceCtx,
    selector: fn(&Profile) -> Option<&T>,
    onchange: fn(&mut Profile, String),
    parts_mapper: fn(&T) -> (&str, &str),
    map_these: Vec<T>,
) -> Element
where
    T: std::fmt::Display + PartialEq,
{
    let tmp = ME.with(|me| me.draft.clone());
    let as_ref = tmp.as_deref().map(selector).flatten();

    rsx! {
        if rcx.editing() {
            div {
                select {
                    class: if as_ref == None { "text-gray-500" },
                    onchange: move |evt| {
                        let v = evt.value();
                        ME.with_mut(|me| onchange(me.draft.as_mut().unwrap(), v))
                    },

                    option { value: "", "{placeholder}" }
                    for val in map_these {
                        option { value: "{val}", selected: as_ref == Some(&val), "{val}" }
                    }

                }
            }
        } else {
            if let Some((emoji, text)) = selector(&pcx.read()).map(parts_mapper) {
                div {
                    "{emoji}"
                    div { "{text}" }
                }
            }
        }
    }
}
