use dioxus::prelude::*;

use crate::modal::{TrModal, MODALS};

type OptCb = Option<Callback<MouseEvent>>;

pub struct ModalBuilder {
    pub(super) class: &'static str,

    pub(super) animated: bool,
    pub(super) h2: Option<&'static str>,
    pub(super) buttons: Vec<(&'static str, OptCb)>,
}

#[allow(dead_code)]
impl ModalBuilder {
    pub fn button(mut self, label: &'static str, callback: OptCb) -> Self {
        self.buttons.push((label, callback));
        self
    }

    pub fn title(mut self, h2: &'static str) -> Self {
        self.h2 = Some(h2);
        self
    }

    pub fn unanimated(mut self) -> Self {
        self.animated = false;
        self
    }

    pub fn message(self, p: &'static str) {
        MODALS.new(
            self.class,
            self.animated,
            rsx! {
                if let Some(header) = self.h2 {
                    h2 { "{header}" }
                }

                p { "{p}" }

                div { class: "mt-4 p-2 flex gap-2 justify-evenly",
                    for (label , ocb) in self.buttons {
                        button {
                            onclick: move |evt| {
                                if let Some(callback) = ocb {
                                    callback(evt.clone());
                                    if evt.propagates() {
                                        MODALS.pop();
                                    }
                                }
                                else{
                                    MODALS.pop();
                                }
                            },
                            "{label}"
                        }
                    }
                }
            },
        )
    }
}
