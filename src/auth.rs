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
    extract::{FromRef, FromRequestParts, OptionalFromRequestParts, Query, State},
    http::{header::SET_COOKIE, HeaderMap},
    response::{IntoResponse, Redirect, Response},
    routing::get,
    RequestPartsExt, Router,
};
use axum_extra::{headers, typed_header::TypedHeaderRejectionReason, TypedHeader};
use dioxus::logger::tracing;
use http::{header, request::Parts, StatusCode};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet,
    RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::{convert::Infallible, env};

static COOKIE_NAME: &str = "SESSION";

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
        .route("/protected", get(protected))
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
    let redirect_url = env::var("REDIRECT_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8080/auth/authorized".to_string());

    let auth_url = env::var("AUTH_URL").unwrap_or_else(|_| {
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

// The user data we'll get back from Discord.
// https://discord.com/developers/docs/resources/user#user-object-user-structure
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    avatar: Option<String>,
    username: String,
    discriminator: String,
}

async fn discord_auth(
    State(client): State<BasicClient>,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .url();

    let session_id = CsrfToken::new_random().secret().to_string();
    sqlx::query(
        "
        INSERT INTO auth_sessions (id, csrf_token, expires_at)
        VALUES ($1, $2, NOW() + INTERVAL '15 minutes')
        ",
    )
    .bind(&session_id)
    .bind(csrf_token.secret())
    .execute(&pool)
    .await
    .context("failed to store csrf session")?;

    // Attach the session cookie to the response header
    let cookie = format!("{COOKIE_NAME}={session_id}; SameSite=Lax; HttpOnly; Secure; Path=/");
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        cookie.parse().context("failed to parse cookie")?,
    );

    Ok((headers, Redirect::to(auth_url.as_ref())))
}

// Valid user session required. If there is none, redirect to the auth page
async fn protected(user: User) -> impl IntoResponse {
    format!("Welcome to the protected area :)\nHere's your info:\n{user:?}")
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
    let user_data: User = client
        // https://discord.com/developers/docs/resources/user#get-current-user
        .get("https://discordapp.com/api/users/@me")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .context("failed in sending request to target Url")?
        .json::<User>()
        .await
        .context("failed to deserialize response as JSON")?;

    let session_id = cookies
        .get(COOKIE_NAME)
        .context("unexpected error getting cookie name")?;

    let updated = sqlx::query(
        "
        UPDATE auth_sessions
        SET
            discord_user_id = $2,
            avatar = $3,
            username = $4,
            discriminator = $5,
            expires_at = NOW() + INTERVAL '30 days'
        WHERE id = $1 AND expires_at > NOW()
        ",
    )
    .bind(session_id)
    .bind(&user_data.id)
    .bind(&user_data.avatar)
    .bind(&user_data.username)
    .bind(&user_data.discriminator)
    .execute(&pool)
    .await
    .context("failed to persist user session")?
    .rows_affected();

    if updated == 0 {
        return Err(anyhow!("Session not found or expired").into());
    }

    // Build the cookie
    let cookie = format!("{COOKIE_NAME}={session_id}; SameSite=Lax; HttpOnly; Secure; Path=/");

    // Set cookie
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        cookie.parse().context("failed to parse cookie")?,
    );

    Ok((headers, Redirect::to("/")))
}

struct AuthRedirect;

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary("/auth/discord").into_response()
    }
}

impl<S> FromRequestParts<S> for User
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = AuthRedirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let cookies = parts
            .extract::<TypedHeader<headers::Cookie>>()
            .await
            .map_err(|e| match *e.name() {
                header::COOKIE => match e.reason() {
                    TypedHeaderRejectionReason::Missing => AuthRedirect,
                    _ => panic!("unexpected error getting Cookie header(s): {e}"),
                },
                _ => panic!("unexpected error getting cookies: {e}"),
            })?;
        let session_cookie = cookies.get(COOKIE_NAME).ok_or(AuthRedirect)?;

        let user_row = sqlx::query_as::<_, (String, Option<String>, String, String)>(
            "
            SELECT discord_user_id, avatar, username, discriminator
            FROM auth_sessions
            WHERE id = $1 AND expires_at > NOW() AND discord_user_id IS NOT NULL
            ",
        )
        .bind(session_cookie)
        .fetch_optional(&pool)
        .await
        .map_err(|_| AuthRedirect)?
        .ok_or(AuthRedirect)?;

        let user = User {
            id: user_row.0,
            avatar: user_row.1,
            username: user_row.2,
            discriminator: user_row.3,
        };

        Ok(user)
    }
}

impl<S> OptionalFromRequestParts<S> for User
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        match <User as FromRequestParts<S>>::from_request_parts(parts, state).await {
            Ok(res) => Ok(Some(res)),
            Err(AuthRedirect) => Ok(None),
        }
    }
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
