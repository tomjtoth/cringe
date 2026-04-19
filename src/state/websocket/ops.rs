use std::collections::HashMap;

use dioxus::{
    prelude::{debug, error},
    signals::{GlobalSignal, ReadableExt},
};

use crate::views::people::profile::ResourceCtx;

#[derive(Clone, Debug)]
pub enum OpState {
    Success,
    Failure,
}

pub static OPS: GlobalSignal<HashMap<u8, OpState>> = GlobalSignal::new(HashMap::new);

pub fn ws_clear_op_id(rcx: &mut ResourceCtx) {
    let id = rcx.op_id;

    let returned = OPS.with(|ops| {
        debug!("WS op #{id} polled {ops:?}");
        ops.get(&id).cloned()
    });

    if let Some(state) = returned {
        OPS.with_mut(|ops| ops.remove(&id));

        match state {
            OpState::Success => rcx.toggle_editing(),
            OpState::Failure => {
                // TODO: show a modal or simple toast
                error!("WS op #{id} failed");
            }
        };
    }
}
