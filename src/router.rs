use dioxus::prelude::*;

use crate::{
    navbar::Navbar,
    views::{about::About, login::Login, me::Me, people::listing::*},
};

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(Navbar)]
    #[route("/about")]
    About {},

    #[route("/liked")]
    LikedProfiles {},

    #[route("/skipped")]
    SkippedProfiles {},

    #[route("/")]
    SwipeProfiles {},

    #[route("/login")]
    Login {},

    #[route("/me")]
    Me {},

    #[route("/:..segments")]
    CatchAll { segments: Vec<String> },
}

#[component]
fn CatchAll(segments: Vec<String>) -> Element {
    About()
}
