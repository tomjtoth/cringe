use dioxus::prelude::*;

use crate::{
    navbar::Navbar,
    views::{about::About, me::Me, people::listing::*},
};

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    About {},

    #[route("/liked")]
    ListOfLikedProfiles {},

    #[route("/disliked")]
    ListOfDislikedProfiles {},

    #[route("/swipe")]
    #[route("/me")]
    Me {},
}
