use std::collections::HashMap;

use dioxus::{
    fullstack::{use_websocket, CloseCode, WebSocketOptions, Websocket, WebsocketState},
    prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::state::client::{AUTH_CTE, ME};

#[derive(Serialize, Deserialize, Debug)]
pub enum WsResponse {
    ServerAlive,
    ConvertedImageBytes(i32, Vec<u8>, Vec<(i32, String)>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WsRequest {
    KeepAlive,
}

#[get("/api/ws")]
async fn connect_ws(options: WebSocketOptions) -> Result<Websocket<WsRequest, WsResponse>> {
    use crate::state::server::{get_ctx, register_ws, unregister_ws};

    let mut sess_id = None::<String>;

    if let (Some(session_id), pool) = get_ctx().await {
        sess_id = sqlx::query_scalar(&format!(
            r"
            WITH {AUTH_CTE}
            SELECT $1 FROM auth
            "
        ))
        .bind(&session_id)
        .fetch_one(&pool)
        .await?;
    };

    let sess_id = sess_id.ok_or(anyhow::anyhow!(StatusCode::UNAUTHORIZED))?;

    Ok(options.on_upgrade(move |mut socket| async move {
        // determine session for this websocket and register a sender for notifications

        let (tx, mut rx) = tokio::sync::mpsc::channel::<WsResponse>(32);
        register_ws(sess_id.clone(), tx).await;

        loop {
            tokio::select! {
                from_converter = rx.recv() => {
                    if let Some(notification) = from_converter {
                        _ = socket.send(notification).await;
                    }
                }

                from_client = socket.recv() => {
                    match from_client {
                        Ok(WsRequest::KeepAlive) => {
                            _ = socket.send(WsResponse::ServerAlive).await;
                        },
                        Err(e) => {
                            error!("WS error: {e:?}");
                            break;
                        },
                    }
                }
            }
        }

        // cleanup registry when websocket ends
        unregister_ws(&sess_id).await;
    }))
}

async fn sleep(secs: u64) {
    #[cfg(target_arch = "wasm32")]
    gloo_timers::future::sleep(std::time::Duration::from_secs(secs)).await;
}

pub(super) fn init_ws() {
    let mut ws = use_websocket(|| connect_ws(WebSocketOptions::new()));

    use_future(move || async move {
        let init_conn = move || async move {
            match ws.connect().await {
                WebsocketState::FailedToConnect => error!("WS failed to connect"),
                state => info!("WS state: {state:?}"),
            }

            // init keepalive cycle
            if let Err(e) = ws.send(WsRequest::KeepAlive).await {
                error!("WS error: {e:?}");
            }
        };

        init_conn().await;

        while let Ok(from_server) = ws.recv().await {
            match from_server {
                WsResponse::ServerAlive => {
                    sleep(30).await;

                    _ = ws
                        .send(WsRequest::KeepAlive)
                        .await
                        .map_err(move |e| async move {
                            type E = dioxus_fullstack::WebsocketError;

                            match e {
                                E::Capacity
                                | E::ConnectionClosed {
                                    code: CloseCode::Restart,
                                    ..
                                } => {
                                    error!("WS reconnecting after 30 secs: {e:?}");
                                    sleep(30).await;
                                }

                                _ => {
                                    error!("WS reconnecting: {e:?}");
                                }
                            }

                            init_conn().await;
                        })
                }

                WsResponse::ConvertedImageBytes(id, bytes, placeholders) => {
                    info!("Updating #{id}, updated placeholders: {placeholders:?}");

                    let keyed_urls: HashMap<i32, String> =
                        HashMap::from_iter(placeholders.into_iter());

                    if let Some(p) = ME.write().profile.as_mut() {
                        if let Some(i) = p.images.iter_mut().find(|i| i.id() == Some(id)) {
                            i.set_bytes(bytes);
                            i.set_url(None);
                        }

                        for img in p.images.iter_mut() {
                            if let Some(id) = img.id() {
                                if let Some(url) = keyed_urls.get(&id) {
                                    img.set_url(Some(url.clone()));
                                }
                            }
                        }
                    }
                }
            }
        }
    });
}
