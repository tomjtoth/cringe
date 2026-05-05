use dioxus::prelude::*;

use crate::{components::router::Route, views::listing::ListingCtx};

#[component]
pub(super) fn ContextProviders() -> Element {
    let lcx: ListingCtx = use_signal(|| None);
    use_context_provider(|| lcx);

    rsx! {
        Outlet::<Route> {}
    }
}
