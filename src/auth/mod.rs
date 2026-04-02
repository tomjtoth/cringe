use anyhow::{anyhow, Context, Result};
use axum::{
    extract::{Query, State},
    http::{header::SET_COOKIE, HeaderMap},
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Router,
};
use axum_extra::{headers, TypedHeader};
use dioxus::logger::tracing;
use http::StatusCode;
use oauth2::{CsrfToken, EndpointNotSet, EndpointSet};
use once_cell::sync::Lazy;
use serde::Deserialize;
use sqlx::PgPool;
use std::env;
use strum::IntoEnumIterator;

mod providers;
use providers::Provider;

pub static COOKIE_NAME: &str = "SESSION";

static REDIRECT_URL: &str = "http://127.0.0.1:8080/api/auth/callback";
static REDIRECT_URL_ENV: Lazy<String> =
    Lazy::new(|| env::var("REDIRECT_URL").unwrap_or_else(|_| REDIRECT_URL.to_string()));

type BasicClient = oauth2::basic::BasicClient<
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

pub fn routes(pool: PgPool) -> Result<Router> {
    let mut clients = std::collections::HashMap::new();

    for p in Provider::iter() {
        clients.insert(p, p.oauth_client()?);
    }

    let state = AppState { clients, pool };

    Ok(Router::new()
        .route("/api/auth/{provider}", get(auth_start))
        .route("/api/auth/callback", get(auth_callback))
        .route("/logout", get(logout))
        .with_state(state))
}

#[derive(Clone)]
struct AppState {
    clients: std::collections::HashMap<Provider, BasicClient>,
    pool: PgPool,
}

fn build_session_cookie(session_id: &str) -> String {
    let using_dev_url = *REDIRECT_URL_ENV == REDIRECT_URL;

    format!(
        "{COOKIE_NAME}={session_id}; SameSite=Lax; HttpOnly;{} Path=/",
        if using_dev_url { "" } else { " Secure;" }
    )
}

async fn auth_start(
    axum::extract::Path(provider): axum::extract::Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let provider = Provider::from_str(&provider).ok_or_else(|| anyhow!("invalid provider"))?;

    let client = state.clients.get(&provider).unwrap();

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(provider.scopes())
        .url();

    // 👇 encode provider into state
    let combined_state = format!("{}:{}", provider.as_ref(), csrf_token.secret());

    let session_id = CsrfToken::new_random().secret().to_string();

    sqlx::query(
        "INSERT INTO auth_sessions (id, csrf_token, email, expires_at)
         VALUES ($1, $2, NULL, NOW() + INTERVAL '15 minutes')",
    )
    .bind(&session_id)
    .bind(&combined_state)
    .execute(&state.pool)
    .await?;

    let cookie = build_session_cookie(&session_id);

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.parse()?);

    Ok((headers, Redirect::to(auth_url.as_ref())))
}

async fn logout(
    State(AppState { pool, .. }): State<AppState>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(cookie) = cookies.get(COOKIE_NAME) {
        sqlx::query("DELETE FROM auth_sessions WHERE id = $1")
            .bind(cookie)
            .execute(&pool)
            .await
            .context("failed to delete session")?;
    };

    Ok(Redirect::to("/"))
}

#[derive(Debug, Deserialize)]
struct AuthRequest {
    code: String,
    state: String,
}

async fn csrf_token_validation_workflow(
    auth_request: &AuthRequest,
    session_id: &str,
    pool: &PgPool,
) -> Result<Provider, AppError> {
    let osp = sqlx::query_scalar::<_, String>(
        r#"
        UPDATE auth_sessions
        SET csrf_token = NULL
        WHERE id = $1
        AND expires_at > NOW()
        AND split_part(csrf_token, ':', 2) = $2
        RETURNING split_part(old.csrf_token, ':', 1)
        "#,
    )
    .bind(session_id)
    .bind(&auth_request.state)
    .fetch_optional(pool)
    .await?;

    if let Some(sp) = osp {
        if let Some(p) = Provider::from_str(&sp) {
            return Ok(p);
        }
    }

    Err(anyhow!("CSRF token validation failed!").into())
}

async fn auth_callback(
    Query(query): Query<AuthRequest>,
    State(state): State<AppState>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> Result<impl IntoResponse, AppError> {
    let session_id = cookies
        .get(COOKIE_NAME)
        .ok_or_else(|| anyhow!("missing cookie {COOKIE_NAME}"))?;

    let provider = csrf_token_validation_workflow(&query, session_id, &state.pool).await?;

    let oauth_client = state.clients.get(&provider).unwrap();

    let email = provider.fetch_user_info(&query.code, oauth_client).await?;

    persist_session(session_id, &state.pool, &email).await?;

    let cookie = build_session_cookie(session_id);

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.parse()?);

    Ok((headers, Redirect::to("/")))
}

async fn persist_session(session_id: &str, pool: &PgPool, email: &str) -> Result<(), AppError> {
    let updated = sqlx::query(
        "UPDATE auth_sessions
        SET email = $2, expires_at = NOW() + INTERVAL '30 days'
        WHERE id = $1 AND expires_at > NOW()",
    )
    .bind(session_id)
    .bind(email)
    .execute(pool)
    .await?
    .rows_affected();

    if updated == 0 {
        return Err(anyhow!("Session expired").into());
    }

    Ok(())
}

// Use anyhow, define error and enable '?'
// For a simplified example of using anyhow in axum check /examples/anyhow-error-response
#[derive(Debug)]
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:#}", self.0);

        let body = format!(
            r#"
            <!doctype html>
            <html>
            <head>
                <meta content="text/html;charset=utf-8" http-equiv="Content-Type">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <meta charset="UTF-8">               
                <title>Error</title>
                <link rel="stylesheet" href="{}"/>
                <link rel="icon" href="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'%3E%3Ctext y='0.9em' font-size='90'%3E😱%3C/text%3E%3C/svg%3E">
                <link rel="manifest" href="/manifest.json"/>
            </head>
            <body>
                <div id="main">
                    <div class="app-center text-center">
                        <h1>Something went wrong!</h1>
                        <p>Please try again later.</p>
                    </div>
                </div>
            </body>
            </html>"#,
            crate::TAILWIND_CSS
        );

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::response::Html(body),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
