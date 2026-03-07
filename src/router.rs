use dioxus::prelude::*;

use crate::{
    navbar::Navbar,
    views::{about::About, people::listing::*},
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
    ListOfUncheckedProfiles {},
}
