//! Example OAuth (Discord) implementation.
//!
//! 1) Create a new application at <https://discord.com/developers/applications>
//! 2) Visit the OAuth2 tab to get your AUTH_DISCORD_ID and AUTH_DISCORD_SECRET
//! 3) Add a new redirect URI (for this example: `http://127.0.0.1:3000/auth/authorized`)
//! 4) Run with the following (replacing values appropriately):
//! ```not_rust
//! AUTH_DISCORD_ID=REPLACE_ME AUTH_DISCORD_SECRET=REPLACE_ME cargo run -p example-oauth
//! ```

use anyhow::{anyhow, Context, Result};
use axum::{
    extract::{FromRef, Query, State},
    http::{header::SET_COOKIE, HeaderMap},
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Router,
};
use axum_extra::{headers, TypedHeader};
use dioxus::logger::tracing;
use http::StatusCode;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet,
    RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use sqlx::PgPool;
use std::env;

pub static COOKIE_NAME: &str = "SESSION";
static REDIRECT_URL: &str = "http://127.0.0.1:8080/auth/authorized";

type BasicClient = oauth2::basic::BasicClient<
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

pub fn routes(pool: PgPool) -> Result<Router> {
    let oauth_client = oauth_client()?;
    let app_state = AppState { oauth_client, pool };

    let auth_layer = Router::new()
        .route("/auth/discord", get(discord_auth))
        .route("/auth/authorized", get(login_authorized))
        .route("/logout", get(logout))
        .with_state(app_state);

    Ok(auth_layer)
}

#[derive(Clone)]
struct AppState {
    oauth_client: BasicClient,
    pool: PgPool,
}

impl FromRef<AppState> for BasicClient {
    fn from_ref(state: &AppState) -> Self {
        state.oauth_client.clone()
    }
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

fn oauth_client() -> Result<BasicClient> {
    // Environment variables (* = required):
    // *"AUTH_DISCORD_ID"     "REPLACE_ME";
    // *"AUTH_DISCORD_SECRET" "REPLACE_ME";
    //  "REDIRECT_URL"  "http://127.0.0.1:3000/auth/authorized";
    //  "AUTH_URL"      "https://discord.com/api/oauth2/authorize?response_type=code";
    //  "TOKEN_URL"     "https://discord.com/api/oauth2/token";

    let client_id = env::var("AUTH_DISCORD_ID").context("Missing AUTH_DISCORD_ID!")?;
    let client_secret = env::var("AUTH_DISCORD_SECRET").context("Missing AUTH_DISCORD_SECRET!")?;
    let redirect_url = env::var("REDIRECT_URL").unwrap_or_else(|_| REDIRECT_URL.to_string());

    let auth_url = env::var("AUTH_DISCORD_URL").unwrap_or_else(|_| {
        "https://discord.com/api/oauth2/authorize?response_type=code".to_string()
    });

    let token_url = env::var("TOKEN_URL")
        .unwrap_or_else(|_| "https://discord.com/api/oauth2/token".to_string());

    Ok(oauth2::basic::BasicClient::new(ClientId::new(client_id))
        .set_client_secret(ClientSecret::new(client_secret))
        .set_auth_uri(
            AuthUrl::new(auth_url).context("failed to create new authorization server URL")?,
        )
        .set_token_uri(TokenUrl::new(token_url).context("failed to create new token endpoint URL")?)
        .set_redirect_uri(
            RedirectUrl::new(redirect_url).context("failed to create new redirection URL")?,
        ))
}

fn build_session_cookie(session_id: &str) -> String {
    let redirect_url = std::env::var("REDIRECT_URL").unwrap_or_else(|_| REDIRECT_URL.to_string());
    let using_dev_url = redirect_url == REDIRECT_URL;

    format!(
        "{COOKIE_NAME}={session_id}; SameSite=Lax; HttpOnly;{} Path=/",
        if using_dev_url { "" } else { " Secure;" }
    )
}

// Discord may omit email-related fields even when the email scope is requested.
#[derive(Debug, Deserialize)]
struct DiscordUser {
    email: Option<String>,
    verified: Option<bool>,
}

async fn discord_auth(
    State(client): State<BasicClient>,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_string()))
        .url();

    let session_id = CsrfToken::new_random().secret().to_string();
    sqlx::query(
        "
        INSERT INTO auth_sessions (id, csrf_token, email, expires_at)
        VALUES ($1, $2, NULL, NOW() + INTERVAL '15 minutes')
        ",
    )
    .bind(&session_id)
    .bind(csrf_token.secret())
    .execute(&pool)
    .await
    .context("failed to store csrf session")?;

    // Attach the session cookie to the response header
    let cookie = build_session_cookie(&session_id);
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        cookie.parse().context("failed to parse cookie")?,
    );

    Ok((headers, Redirect::to(auth_url.as_ref())))
}

async fn logout(
    State(pool): State<PgPool>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> Result<impl IntoResponse, AppError> {
    let Some(cookie) = cookies.get(COOKIE_NAME) else {
        return Ok(Redirect::to("/"));
    };

    sqlx::query("DELETE FROM auth_sessions WHERE id = $1")
        .bind(cookie)
        .execute(&pool)
        .await
        .context("failed to delete session")?;

    Ok(Redirect::to("/"))
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AuthRequest {
    code: String,
    state: String,
}

async fn csrf_token_validation_workflow(
    auth_request: &AuthRequest,
    cookies: &headers::Cookie,
    pool: &PgPool,
) -> Result<(), AppError> {
    // Extract the cookie from the request
    let cookie = cookies
        .get(COOKIE_NAME)
        .context("unexpected error getting cookie name")?
        .to_string();

    let stored_csrf_token: Option<String> = sqlx::query_scalar(
        "SELECT csrf_token FROM auth_sessions WHERE id = $1 AND expires_at > NOW()",
    )
    .bind(&cookie)
    .fetch_optional(pool)
    .await
    .context("failed to load csrf session")?;

    let stored_csrf_token = stored_csrf_token.ok_or_else(|| anyhow!("Session not found"))?;

    // Validate CSRF token is the same as the one in the auth request
    if stored_csrf_token != auth_request.state {
        return Err(anyhow!("CSRF token mismatch").into());
    }

    sqlx::query("UPDATE auth_sessions SET csrf_token = NULL WHERE id = $1")
        .bind(&cookie)
        .execute(pool)
        .await
        .context("failed to consume csrf token")?;

    Ok(())
}

async fn login_authorized(
    Query(query): Query<AuthRequest>,
    State(pool): State<PgPool>,
    State(oauth_client): State<BasicClient>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> Result<impl IntoResponse, AppError> {
    csrf_token_validation_workflow(&query, &cookies, &pool).await?;

    let client = oauth2::reqwest::Client::new();

    // Get an auth token
    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(&client)
        .await
        .context("failed in sending request request to authorization server")?;

    // Fetch user data from discord
    let user_data = client
        // https://discord.com/developers/docs/resources/user#get-current-user
        .get("https://discordapp.com/api/users/@me")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .context("failed in sending request to target Url")?
        .json::<DiscordUser>()
        .await
        .context("failed to deserialize response as JSON")?;

    let email = user_data
        .email
        .ok_or_else(|| anyhow!("Discord account did not provide an email"))?;

    if !user_data.verified.unwrap_or(false) {
        return Err(anyhow!("Discord account email is not verified").into());
    }

    let session_id = cookies
        .get(COOKIE_NAME)
        .context("unexpected error getting cookie name")?;

    let updated = sqlx::query(
        "
        UPDATE auth_sessions
        SET
            email = $2,
            expires_at = NOW() + INTERVAL '30 days'
        WHERE id = $1 AND expires_at > NOW()
        ",
    )
    .bind(session_id)
    .bind(&email)
    .execute(&pool)
    .await
    .context("failed to persist user session")?
    .rows_affected();

    if updated == 0 {
        return Err(anyhow!("Session not found or expired").into());
    }

    // Build and set the cookie
    let cookie = build_session_cookie(session_id);
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        cookie.parse().context("failed to parse cookie")?,
    );

    Ok((headers, Redirect::to("/")))
}

// Use anyhow, define error and enable '?'
// For a simplified example of using anyhow in axum check /examples/anyhow-error-response
#[derive(Debug)]
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:#}", self.0);

        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
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
