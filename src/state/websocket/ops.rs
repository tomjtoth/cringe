use std::collections::HashMap;

use dioxus::{prelude::debug, signals::GlobalSignal};

use crate::state::websocket::WsRequest;

#[derive(Debug)]
pub enum OpState {
    Pending,
    Success,
    Failure,
}

pub static OPS: GlobalSignal<HashMap<u32, OpState>> = GlobalSignal::new(HashMap::new);

pub(super) fn register(req: &WsRequest) {
    match req {
        WsRequest::ImageOp(oid, ..) | WsRequest::PromptOp(oid, ..) => {
            OPS.with_mut(|ops| {
                ops.insert(*oid, OpState::Pending);
                debug!("WS registered operation id {oid} ({:?})", ops);
            });
        }

        _ => (),
    }
}
