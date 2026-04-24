mod builder;
pub(super) mod components;

use dioxus::prelude::*;

use crate::modal::builder::ModalBuilder;

type GsModal = GlobalSignal<Vec<(u8, String, bool, Element)>>;
pub static MODALS: GsModal = GlobalSignal::new(|| vec![]);

#[allow(dead_code)]
pub trait TrModal {
    fn build(&self, class: &'static str) -> ModalBuilder;
    fn new(&self, class: &str, animated: bool, element: Element);
    fn pop(&self);
    fn truncate(&self, len: usize);
}

impl TrModal for GsModal {
    fn build(&self, class: &'static str) -> ModalBuilder {
        ModalBuilder {
            class,
            animated: true,
            h2: None,
            buttons: vec![],
        }
    }

    fn new(&self, cls: &str, animated: bool, elem: Element) {
        self.with_mut(move |arr| arr.push((arr.len() as u8, cls.to_string(), animated, elem)));
    }

    fn pop(&self) {
        self.with_mut(|arr| arr.pop());
    }

    fn truncate(&self, len: usize) {
        self.with_mut(|arr| arr.truncate(len));
    }
}
