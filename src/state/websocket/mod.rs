pub mod ops;
#[cfg(feature = "server")]
pub mod server;

use dioxus::{
    fullstack::{
        use_websocket,
        CloseCode::{Away, Normal},
        UseWebsocket, WebSocketOptions, Websocket,
        WebsocketError::{self, ConnectionClosed},
    },
    prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::{
    models::{image::Image, person::Prompt},
    router::Route,
    state::{
        image::{handle_image_crud_res, image_cli_converted, ImageConversionResult, ImageOpResult},
        image::{
            handle_conversion_res, handle_image_crud_res, ImageConversionResult, ImageOpResult,
        },
        prompt::{handle_prompt_crud_res, PromptOpResult},
        AUTH_CTE, ME,
    },
    utils::sleep,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WsResponse {
    ServerAlive,
    PromptOp(u128, PromptOpResult),
    ImageOp(u128, ImageOpResult),
    ImageConversion(ImageConversionResult),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WsRequest {
    KeepAlive,
    PromptOp(u128, Prompt),
    ImageOp(u128, Image),
}

#[get("/api/ws")]
async fn ws_endpoint(options: WebSocketOptions) -> Result<Websocket<WsRequest, WsResponse>> {
    use crate::state::{
        server::{get_ctx, ServerCtx},
        websocket::server::{ws_notify, ws_register, ws_unregister},
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

    /// rustfmt did not work within tokio::select! macro
    async fn handle_req(
        ctx: &ServerCtx,
        req: WsRequest,
        socket: &mut dioxus_fullstack::TypedWebsocket<WsRequest, WsResponse>,
    ) {
        if !matches!(req, WsRequest::KeepAlive) {
            info!("Received {req:?}");
        }

        let target = |authorized: bool| {
            if authorized {
                None
            } else {
                Some(vec![ctx.session_id.clone()])
            }
        };

        match req {
            WsRequest::KeepAlive => {
                _ = socket.send(WsResponse::ServerAlive).await;
            }

            WsRequest::PromptOp(oid, prompt) => match prompt_crud(&ctx, prompt).await {
                Ok(res) => ws_notify(target(res.authorized), WsResponse::PromptOp(oid, res)).await,
                Err(e) => error!("PromptOp failed: {e:?}"),
            },

            WsRequest::ImageOp(oid, img) => match image_crud(&ctx, img).await {
                Ok(res) => ws_notify(target(res.authorized), WsResponse::ImageOp(oid, res)).await,
                Err(e) => error!("ImageOp failed: {e:?}"),
            },
        }
    }

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
                        Ok(req) => handle_req(&ctx, req, &mut socket).await,

                        // this gets logged in ws_unregister
                        Err(ConnectionClosed { code: Normal, .. })
                        | Err(ConnectionClosed { code: Away, .. }) => break,

                        Err(e) => {
                            error!("WS error: {e:?}");
                            break;
                        }
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
    pub async fn req(&self, request: WsRequest) -> Result<(), WebsocketError> {
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

                WsResponse::PromptOp(op_id, res) => handle_prompt_crud_res(op_id, res),

                WsResponse::ImageOp(op_id, res) => handle_image_crud_res(op_id, res),

                WsResponse::ImageConversion(res) => handle_conversion_res(res),
            }
        }

        if ME.read().authenticated {
            info!("WS connection #{} failed, reconnecting...", state());

            sleep(1).await;
            state.with_mut(|s| *s = s.wrapping_add(1));
        }
    });
}
