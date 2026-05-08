use dioxus::prelude::*;

use crate::{
    components::{context_providers::ContextProviders, modal::ModalRenderer, protector::Protector},
    views::{listing::Listing, matches::Matches, me::Me},
};

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(ContextProviders)]
    #[layout(ModalRenderer)]
    #[layout(Protector)]
    #[route("/")]
    Listing {},

    #[route("/matches")]
    Matches {},

    #[route("/me")]
    Me {},

    #[route("/:..segments")]
    CatchAll { segments: Vec<String> },
}

#[component]
fn CatchAll(segments: Vec<String>) -> Element {
    let navi = use_navigator();

    navi.replace(Route::Listing {});

    rsx! {
        h1 { "Redirecting..." }
    }
}
