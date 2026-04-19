use dioxus::prelude::*;

use crate::{models::person::Person, views::people::profile::details::DetailsCtx};

mod has;
mod wants;

#[component]
pub(super) fn Kids() -> Element {
    let dcx = use_context::<DetailsCtx>();
    let already_has_kids = dcx.ro.read().kids.as_ref().is_some_and(|k| k.has > Some(0));

    rsx! {
        has::Has { already_has_kids }
        wants::Wants { already_has_kids }
    }
}
