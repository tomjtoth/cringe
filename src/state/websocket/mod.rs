pub mod ops;
#[cfg(feature = "server")]
pub mod server;

use dioxus::{
    fullstack::{use_websocket, UseWebsocket, WebSocketOptions, Websocket, WebsocketError},
    prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::{
    models::image::Image,
    router::Route,
    state::{
        image::{image_cli_converted, image_cli_ops, ImageConversionResult, ImageOpResult},
        AUTH_CTE, ME,
    },
    utils::sleep,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WsResponse {
    ServerAlive,
    ImageOp(u32, ImageOpResult),
    ImageConversion(ImageConversionResult),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WsRequest {
    KeepAlive,
    ImageOp(u32, Image),
}

#[get("/api/ws")]
async fn ws_endpoint(options: WebSocketOptions) -> Result<Websocket<WsRequest, WsResponse>> {
    use crate::state::{
        server::{get_ctx, ServerCtx},
        websocket::server::{ws_register, ws_unregister},
    };

    let (mut sess_id, pool) = get_ctx().await;

    if let Some(session_id) = sess_id {
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

        let ctx = ServerCtx {
            session_id: sess_id.clone(),
            pool,
        };

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

                        Ok(WsRequest::ImageOp(oid, img)) => {
                            if let Err(e) = crate::state::image::ops::image_crud_ops(&ctx, oid,img).await {
                                error!("ImageOp failed: {e:?}");
                            }
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

pub struct WsCtx(UseWebsocket<WsRequest, WsResponse>);

impl Copy for WsCtx {}
impl Clone for WsCtx {
    fn clone(&self) -> Self {
        *self
    }
}
impl std::ops::Deref for WsCtx {
    type Target = UseWebsocket<WsRequest, WsResponse>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl WsCtx {
    /// #### Registers outgoing requests' operation id and logs errors
    pub async fn req(&self, request: WsRequest) -> Result<(), WebsocketError> {
        ops::register(&request);

        self.send(request).await.map_err(|e| {
            error!("WS error: {e:?}");
            e
        })
    }

    pub fn delayed_req(&self, delay: u64, request: WsRequest) {
        let ws = *self;

        spawn(async move {
            sleep(delay).await;

            _ = ws.req(request).await;
        });
    }
}

/// The sole purpose of this component is the re-creation of the WS connection
#[component]
pub fn WsProvider() -> Element {
    let state = use_signal(|| 0u8);

    rsx! {
        if state() % 2 == 0 {
            Even { state, Outlet::<Route> {} }
        } else {
            Odd { state, Outlet::<Route> {} }
        }
    }
}

#[component]
fn Even(state: Signal<u8>, children: Element) -> Element {
    use_ws(state);
    rsx! {
        {children}
    }
}

#[component]
fn Odd(state: Signal<u8>, children: Element) -> Element {
    use_ws(state);
    rsx! {
        {children}
    }
}

fn use_ws(mut state: Signal<u8>) {
    let socket = use_websocket(|| ws_endpoint(WebSocketOptions::new().with_automatic_reconnect()));

    info!("WS connection #{}: {:?}", state(), socket.status());

    let mut ws = use_context_provider(|| WsCtx(socket));

    use_future(move || async move {
        ws.delayed_req(30, WsRequest::KeepAlive);

        while let Ok(from_server) = ws.0.recv().await {
            if !matches!(from_server, WsResponse::ServerAlive) {
                info!("Received {from_server:?}");
            }

            match from_server {
                WsResponse::ServerAlive => ws.delayed_req(30, WsRequest::KeepAlive),

                WsResponse::ImageOp(oid, res) => image_cli_ops(oid, res),

                WsResponse::ImageConversion(res) => image_cli_converted(res),
            }
        }

        if ME.read().authenticated {
            info!("WS connection #{} failed, reconnecting...", state());

            sleep(1).await;
            state.with_mut(|s| *s = s.wrapping_add(1));
        }
    });
}
