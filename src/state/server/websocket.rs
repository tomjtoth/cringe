use std::collections::HashMap;

use dioxus::prelude::info;
use once_cell::sync::Lazy;
use tokio::sync::{mpsc::Sender, Mutex};

use crate::state::websocket::WsResponse;

// Registry of websocket senders keyed by session id.
static WS_REG: Lazy<Mutex<HashMap<String, Sender<WsResponse>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub async fn ws_notify(session_id: &String, noti: WsResponse) {
    if let Some(tx) = WS_REG.lock().await.get(session_id) {
        _ = tx.send(noti).await
    }
}

pub async fn ws_register(session_id: String, tx: Sender<WsResponse>) {
    let mut reg = WS_REG.lock().await;
    info!("WS registering: {session_id}");
    reg.insert(session_id, tx);
}

pub async fn ws_unregister(session_id: &str) {
    let mut reg = WS_REG.lock().await;
    info!("WS unregistering: {session_id}");
    reg.remove(session_id);
}
