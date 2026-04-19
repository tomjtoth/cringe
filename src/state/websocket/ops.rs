use std::collections::HashMap;

use dioxus::signals::GlobalSignal;

#[derive(Clone, Debug)]
pub enum OpState {
    Success,
    Failure,
}

pub static OPS: GlobalSignal<HashMap<u8, OpState>> = GlobalSignal::new(HashMap::new);
