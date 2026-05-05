use dioxus::prelude::*;

use crate::{
    components::{modal::ModalRenderer, protector::Protector},
    views::{matches::Matches, me::Me, people::listing::Cringe},
};

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(ModalRenderer)]
    #[layout(Protector)]
    #[route("/")]
    Cringe {},

    #[route("/matches")]
    Matches {},

    #[route("/me")]
    Me {},

    #[route("/:..segments")]
    CatchAll { segments: Vec<String> },
}

#[component]
fn CatchAll(segments: Vec<String>) -> Element {
    Cringe()
}
