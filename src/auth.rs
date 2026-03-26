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
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet,
    RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use once_cell::sync::Lazy;
use serde::Deserialize;
use sqlx::PgPool;
use std::env;

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

    for p in [Provider::Discord, Provider::Google, Provider::Github] {
        clients.insert(p, oauth_client(p)?);
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Provider {
    Discord,
    Google,
    Github,
}

impl Provider {
    fn as_str(&self) -> &'static str {
        match self {
            Provider::Discord => "discord",
            Provider::Google => "google",
            Provider::Github => "github",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "discord" => Some(Self::Discord),
            "google" => Some(Self::Google),
            "github" => Some(Self::Github),
            _ => None,
        }
    }

    fn scopes(&self) -> Vec<Scope> {
        match self {
            Provider::Google => vec![
                Scope::new("openid".into()),
                Scope::new("email".into()),
                Scope::new("profile".into()),
            ],
            Provider::Github => vec![Scope::new("user:email".into())],
            Provider::Discord => vec![Scope::new("email".into())],
        }
    }

    async fn fetch_email(
        &self,
        token: &str,
        http_client: &oauth2::reqwest::Client,
    ) -> Result<String> {
        #[derive(Deserialize)]
        struct ProviderResponse {
            email: Option<String>,
            verified: Option<bool>,
            email_verified: Option<bool>,
        }

        let url = match self {
            Provider::Discord => "https://discord.com/api/users/@me",
            Provider::Google => "https://openidconnect.googleapis.com/v1/userinfo",
            Provider::Github => "https://api.github.com/user",
        };

        let provider_response: ProviderResponse = http_client
            .get(url)
            .bearer_auth(token)
            .header("User-Agent", "cringe-backend")
            .send()
            .await?
            .json()
            .await?;

        match self {
            Provider::Discord => {
                if let ProviderResponse {
                    email: Some(email),
                    verified: Some(true),
                    ..
                } = provider_response
                {
                    return Ok(email);
                }
            }

            Provider::Google => {
                if let ProviderResponse {
                    email: Some(email),
                    email_verified: Some(true),
                    ..
                } = provider_response
                {
                    return Ok(email);
                }
            }

            Provider::Github => {
                if let ProviderResponse {
                    email: Some(email), ..
                } = provider_response
                {
                    return Ok(email);
                }
            }
        };

        Err(anyhow!("no verified email"))
    }
}

fn oauth_client(provider: Provider) -> Result<BasicClient> {
    let (id_env, secret_env, auth_url, token_url) = match provider {
        Provider::Discord => (
            "AUTH_DISCORD_ID",
            "AUTH_DISCORD_SECRET",
            "https://discord.com/api/oauth2/authorize",
            "https://discord.com/api/oauth2/token",
        ),

        Provider::Google => (
            "AUTH_GOOGLE_ID",
            "AUTH_GOOGLE_SECRET",
            "https://accounts.google.com/o/oauth2/v2/auth",
            "https://oauth2.googleapis.com/token",
        ),

        Provider::Github => (
            "AUTH_GITHUB_ID",
            "AUTH_GITHUB_SECRET",
            "https://github.com/login/oauth/authorize",
            "https://github.com/login/oauth/access_token",
        ),
    };

    let client_id = env::var(id_env)?;
    let client_secret = env::var(secret_env)?;

    Ok(oauth2::basic::BasicClient::new(ClientId::new(client_id))
        .set_client_secret(ClientSecret::new(client_secret))
        .set_auth_uri(AuthUrl::new(auth_url.to_string())?)
        .set_token_uri(TokenUrl::new(token_url.to_string())?)
        .set_redirect_uri(RedirectUrl::new(REDIRECT_URL_ENV.to_owned())?))
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
    let combined_state = format!("{}:{}", provider.as_str(), csrf_token.secret());

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
#[allow(dead_code)]
struct AuthRequest {
    code: String,
    state: String,
}

async fn csrf_token_validation_workflow(
    auth_request: &AuthRequest,
    session_id: &str,
    pool: &PgPool,
) -> Result<Provider, AppError> {
    let stored: String = sqlx::query_scalar(
        "SELECT csrf_token FROM auth_sessions WHERE id = $1 AND expires_at > NOW()",
    )
    .bind(&session_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| anyhow!("session not found"))?;

    let (stored_provider, stored_csrf) = stored
        .split_once(':')
        .ok_or_else(|| anyhow!("invalid stored csrf"))?;

    if stored_csrf != auth_request.state {
        return Err(anyhow!("CSRF token mismatch").into());
    }

    sqlx::query("UPDATE auth_sessions SET csrf_token = NULL WHERE id = $1")
        .bind(&session_id)
        .execute(pool)
        .await?;

    let provider = Provider::from_str(stored_provider)
        .ok_or_else(|| anyhow!("could not parse provider from str"))?;

    Ok(provider)
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
    let http_client = oauth2::reqwest::Client::new();

    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(&http_client)
        .await?;

    let email = provider
        .fetch_email(token.access_token().secret(), &http_client)
        .await?;

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
