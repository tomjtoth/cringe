use dioxus::prelude::*;

use crate::models::person::Person;

mod has;
mod wants;

#[component]
pub(super) fn Kids(sig: Signal<Person>, editing: bool) -> Element {
    let already_has_kids = sig.read().kids.as_ref().is_some_and(|k| k.has > Some(0));

    rsx! {
        has::Has { sig, editing, already_has_kids }
        wants::Wants { sig, editing, already_has_kids }
    }
}
