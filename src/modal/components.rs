use dioxus::prelude::*;

use crate::{
    modal::{TrModal, MODALS},
    router::Route,
};

#[component]
fn Modal(class: String, animated: bool, children: Element) -> Element {
    rsx! {
        div {
            class: "absolute top-0 left-0 h-full w-full {class}",
            class: if animated { "animate-modal-bg" },

            // clicking the blurred background pops the modal
            onclick: move |_| MODALS.pop(),

            div {
                class: "app-center text-center bg-background border rounded p-2",
                class: if animated { "animate-modal-panel" },

                // clicking anything within the bordered dialog should not close
                onclick: move |evt| evt.stop_propagation(),

                {children}
            }
        }

    }
}

#[component]
pub(crate) fn ModalRenderer() -> Element {
    rsx! {
        for (id , class , animated , prompt) in MODALS() {
            Modal { key: "{id}", class, animated, {prompt} }
        }

        Outlet::<Route> {}
    }
}
