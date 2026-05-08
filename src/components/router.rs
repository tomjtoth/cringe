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
    #[redirect("/:.._path", |_path: Vec<String>| Route::Listing {})]
    #[route("/")]
    Listing {},

    #[route("/matches")]
    Matches {},

    #[route("/me")]
    Me {},
}
