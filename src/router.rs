use dioxus::prelude::*;

use crate::{
    navbar::Navbar,
    views::{about::About, me::Me, people::listing::*},
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

    #[route("/me")]
    Me {},
}
