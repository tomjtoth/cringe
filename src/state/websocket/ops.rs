use std::collections::HashMap;

use dioxus::{
    prelude::{debug, error},
    signals::{GlobalSignal, ReadableExt},
};

use crate::{state::websocket::WsRequest, views::people::profile::ResourceCtx};

#[derive(Clone, Debug)]
pub enum OpState {
    Pending,
    Success,
    Failure,
}

pub static OPS: GlobalSignal<HashMap<u128, OpState>> = GlobalSignal::new(HashMap::new);

pub(super) fn ws_register_op_id(req: &WsRequest) {
    match req {
        WsRequest::ImageOp(op_id, ..) | WsRequest::PromptOp(op_id, ..) => {
            OPS.with_mut(|ops| {
                ops.insert(*op_id, OpState::Pending);
                debug!("WS op registered {ops:?}");
            });
        }
        _ => (),
    }
}

pub fn ws_clear_op_id(rcx: &mut ResourceCtx) {
    let id = rcx.op_id();

    let returned = OPS.with(|ops| {
        debug!("WS op #{id} polled {ops:?}");
        ops.get(&id)
            .filter(|state| !matches!(state, OpState::Pending))
            .cloned()
    });

    if let Some(state) = returned {
        OPS.with_mut(|ops| ops.remove(&id));

        match state {
            OpState::Success => rcx.toggle_editing(),
            OpState::Failure => {
                // TODO: show a modal or simple toast
                error!("WS op #{id} failed");
            }
            _ => (),
        };
    }
}
