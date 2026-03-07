use dioxus::prelude::*;

use crate::{navbar::Navbar, views::about::About};

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    About {},

}
