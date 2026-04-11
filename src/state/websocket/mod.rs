#[cfg(feature = "server")]
pub mod server;

use dioxus::{
    fullstack::{
        use_websocket, CloseCode, UseWebsocket, WebSocketOptions, Websocket, WebsocketError,
        WebsocketState,
    },
    prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::{
    models::image::Image,
    state::{
        image::{ImageConversionResult, ImageOpResult},
        AUTH_CTE, ME,
    },
    utils::sleep,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WsResponse {
    ServerAlive,
    ImageOp(ImageOpResult),
    ImageConversion(ImageConversionResult),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WsRequest {
    KeepAlive,
    ImageOp(Image),
}

#[get("/api/ws")]
async fn ws_endpoint(options: WebSocketOptions) -> Result<Websocket<WsRequest, WsResponse>> {
    use crate::state::{
        server::get_ctx,
        websocket::server::{ws_register, ws_unregister},
    };

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
        ws_register(sess_id.clone(), tx).await;

        loop {
            tokio::select! {
                from_server_fns = rx.recv() => {
                    if let Some(notification) = from_server_fns {
                        _ = socket.send(notification).await;
                    }
                }

                from_client = socket.recv() => {
                    match from_client {
                        Ok(WsRequest::KeepAlive) => {
                            _ = socket.send(WsResponse::ServerAlive).await;
                        },

                        Ok(WsRequest::ImageOp(img)) => {
                            info!("received img, sized: {:?}", img.bytes().as_ref().map(|b|b.len()));
                            _ = crate::state::image::image_op(img).await;
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
        ws_unregister(&sess_id).await;
    }))
}

pub type WsCtx = UseWebsocket<WsRequest, WsResponse>;

pub async fn ws_send(sock: WsCtx, payload: WsRequest) -> Result<(), WebsocketError> {
    sock.send(payload).await
}

pub(super) fn ws_init() {
    let mut ws = use_websocket(|| ws_endpoint(WebSocketOptions::new().with_automatic_reconnect()));

    use_context_provider(|| ws);

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

        let error_handler = move |e| async move {
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
        };

        while let Ok(from_server) = ws.recv().await {
            match from_server {
                WsResponse::ServerAlive => {
                    sleep(30).await;

                    _ = ws.send(WsRequest::KeepAlive).await.map_err(error_handler)
                }

                WsResponse::ImageOp(ImageOpResult {
                    authorized,
                    image,
                    sorted,
                }) => {}

                WsResponse::ImageConversion(ImageConversionResult {
                    image,
                    placeholders,
                }) => {
                    if let Some(p) = ME.write().profile.as_mut() {
                        if placeholders.len() > 0 {
                            info!("Updating placeholders");
                            for img in p.images.iter_mut() {
                                if let Some(id) = img.id() {
                                    if let Some(url) = placeholders.get(&id) {
                                        img.set_url(Some(url.clone()));
                                    }
                                }
                            }
                        }

                        if *image.user_id() == p.id {
                            if let Some(id) = image.id() {
                                info!("Received #{id}");
                            }
                            if let Some(i) = p.images.iter_mut().find(|i| i.id() == image.id()) {
                                *i = image;
                            }
                        }
                    }
                }
            }
        }
    });
}
