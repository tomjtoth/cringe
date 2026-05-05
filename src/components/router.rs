use dioxus::prelude::*;

use crate::{
    components::{modal::ModalRenderer, protector::Protector},
    views::{listing::Listing, matches::Matches, me::Me},
};

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
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
    Listing()
}
