use dioxus::prelude::*;

use crate::components::{
    login::Login,
    modal::{TrModal, MODALS},
};

#[component]
pub fn Showcase() -> Element {
    rsx! {
        button {
            onclick: move |_| MODALS.new("z-10", true, rsx! {
                Login {}
            }),

            "Login"
        }
    }
}
