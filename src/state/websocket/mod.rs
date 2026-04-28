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
    modal::{TrModal, MODALS},
    models::{Image, Profile, Prompt},
    state::{
        details::{handle_details_update_res, DetailsUpateRes},
        image::{
            handle_conversion_res, handle_image_crud_res, ImageConversionResult, ImageOpResult,
        },
        prompt::{handle_prompt_crud_res, PromptOpResult},
        AUTH_CTE,
    },
    utils::sleep,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WsResponse {
    ServerAlive,
    DetailsUpdate(u8, DetailsUpateRes),
    PromptOp(u8, PromptOpResult),
    ImageOp(u8, ImageOpResult),
    ImageConversion(ImageConversionResult),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WsRequest {
    KeepAlive,
    DetailsUpdate(u8, Profile),
    PromptOp(u8, Prompt),
    ImageOp(u8, Image),
}

#[cfg(debug_assertions)]
type WsEncoding = dioxus_fullstack::JsonEncoding;
#[cfg(not(debug_assertions))]
type WsEncoding = dioxus_fullstack::CborEncoding;

#[get("/api/ws")]
async fn ws_endpoint(
    options: WebSocketOptions,
) -> Result<Websocket<WsRequest, WsResponse, WsEncoding>> {
    use crate::state::{
        image::ops::image_crud,
        prompt::crud::prompt_crud,
        server::{get_ctx, ServerCtx},
        websocket::server::{ws_notify, ws_register, ws_unregister},
    };

    type TypedWs = dioxus_fullstack::TypedWebsocket<WsRequest, WsResponse, WsEncoding>;

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
    async fn handle_req(ctx: &ServerCtx, req: WsRequest, socket: &mut TypedWs) {
        use crate::state::details::server::update_details;

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

            WsRequest::DetailsUpdate(op_id, me) => match update_details(&ctx, me).await {
                Ok(res) => {
                    ws_notify(
                        target(res.authorized),
                        WsResponse::DetailsUpdate(op_id, res),
                    )
                    .await
                }
                Err(e) => error!("DetailsUpdate failed: {e:?}"),
            },

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

    Ok(options.on_upgrade(move |mut socket: TypedWs| async move {
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

pub struct WsCtx(UseWebsocket<WsRequest, WsResponse, WsEncoding>);

impl Copy for WsCtx {}
impl Clone for WsCtx {
    fn clone(&self) -> Self {
        *self
    }
}
impl std::ops::Deref for WsCtx {
    type Target = UseWebsocket<WsRequest, WsResponse, WsEncoding>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl WsCtx {
    pub async fn req(&self, request: WsRequest) -> Result<(), WebsocketError> {
        let show_modal = !matches!(&request, &WsRequest::KeepAlive);

        self.send(request).await.map_err(|e| {
            error!("WS error: {e:?}");

            if show_modal {
                MODALS
                    .build("z-10")
                    .title("WS op failed")
                    .button("Ok", None)
                    .code(&format!("{:?}", e));
            }

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

pub(super) fn use_ws() {
    let socket = use_websocket(|| ws_endpoint(WebSocketOptions::new().with_automatic_reconnect()));

    let mut ws = use_context_provider(|| WsCtx(socket));

    let _reload_now = use_callback(|evt: MouseEvent| {
        evt.stop_propagation();

        #[cfg(all(not(debug_assertions), target_arch = "wasm32"))]
        if let Some(window) = web_sys::window() {
            _ = window.location().reload();
        }
    });

    use_future(move || async move {
        ws.delayed_req(30, WsRequest::KeepAlive);

        while let Ok(from_server) = ws.0.recv().await {
            if !matches!(from_server, WsResponse::ServerAlive) {
                info!("Received {from_server:?}");
            }

            match from_server {
                WsResponse::ServerAlive => ws.delayed_req(30, WsRequest::KeepAlive),

                WsResponse::DetailsUpdate(op_id, res) => handle_details_update_res(op_id, res),

                WsResponse::PromptOp(op_id, res) => handle_prompt_crud_res(op_id, res),

                WsResponse::ImageOp(op_id, res) => handle_image_crud_res(op_id, res),

                WsResponse::ImageConversion(res) => handle_conversion_res(res),
            }
        }

        #[cfg(all(not(debug_assertions), target_arch = "wasm32"))]
        if super::ME.read().authenticated {
            info!("WS connection failed, reconnecting in a sec...");

            MODALS
                .build("z-100")
                .title("WebSocket connection failed")
                .button("Reload page now", Some(_reload_now))
                .p("Reloading page in 3 seconds...");

            sleep(3).await;
            if let Some(window) = web_sys::window() {
                _ = window.location().reload();
            }
        }
    });
}
