use std::collections::HashMap;

use dioxus::prelude::info;
use once_cell::sync::Lazy;
use tokio::sync::{mpsc::Sender, Mutex};

use crate::state::websocket::WsResponse;

// Registry of websocket senders keyed by session id.
static WS_REG: Lazy<Mutex<HashMap<String, Sender<WsResponse>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// if no specific connections were defined, it'll be a broadcast to all available connections
pub async fn ws_notify(specific_connections: Option<Vec<String>>, notification: WsResponse) {
    let mut recipients = vec![];

    {
        let guard = WS_REG.lock().await;

        if let Some(specific_connections) = specific_connections {
            for conn in specific_connections {
                if let Some(tx) = guard.get(&conn).cloned() {
                    recipients.push(tx)
                }
            }
        } else {
            for tx in guard.values() {
                recipients.push(tx.clone())
            }
        }
    }

    for tx in recipients {
        _ = tx.send(notification.clone()).await
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
