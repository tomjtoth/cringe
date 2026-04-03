use anyhow::{anyhow, Context, Result};
use dioxus::logger::tracing;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use std::env;

use strum::{EnumProperty, IntoEnumIterator};
use strum_macros::{AsRefStr, EnumIter, EnumProperty};

use crate::auth::{BasicClient, REDIRECT_URL_ENV};

#[derive(
    EnumIter, AsRefStr, EnumProperty, Clone, Copy, Debug, PartialEq, Eq, Hash, strum_macros::Display,
)]
#[strum(serialize_all = "lowercase")]
pub(super) enum Provider {
    #[strum(props(
        AUTH_URL = "https://discord.com/api/oauth2/authorize",
        TOKEN_URL = "https://discord.com/api/oauth2/token",
        PROFILE_URL = "https://discord.com/api/users/@me",
        scopes = "email"
    ))]
    Discord,

    #[strum(props(
        AUTH_URL = "https://www.facebook.com/v25.0/dialog/oauth",
        TOKEN_URL = "https://graph.facebook.com/v25.0/oauth/access_token",
        PROFILE_URL = "https://graph.facebook.com/me?fields=email",
        scopes = "email"
    ))]
    Facebook,

    #[strum(props(
        AUTH_URL = "https://accounts.google.com/o/oauth2/v2/auth",
        TOKEN_URL = "https://oauth2.googleapis.com/token",
        PROFILE_URL = "https://openidconnect.googleapis.com/v1/userinfo",
        scopes = "email"
    ))]
    Google,

    #[strum(props(
        AUTH_URL = "https://github.com/login/oauth/authorize",
        TOKEN_URL = "https://github.com/login/oauth/access_token",
        PROFILE_URL = "https://api.github.com/user/emails",
        scopes = "user:email"
    ))]
    Github,

    #[strum(props(
        AUTH_URL = "https://www.strava.com/oauth/authorize",
        TOKEN_URL = "https://www.strava.com/oauth/token",
        PROFILE_URL = "https://www.strava.com/api/v3/athlete",
        scopes = "read"
    ))]
    Strava,
}

impl Provider {
    pub(super) fn from_str(s: &str) -> Option<Self> {
        Provider::iter().find(|p| p.as_ref() == s)
    }

    pub(super) fn scopes(&self) -> Vec<Scope> {
        self.get_str("scopes")
            .map(|ss| ss.split(',').map(|s| Scope::new(s.into())).collect())
            .unwrap_or_default()
    }

    fn auth_and_token_urls(&self) -> (&str, &str) {
        (
            self.get_str("AUTH_URL").unwrap(),
            self.get_str("TOKEN_URL").unwrap(),
        )
    }

    fn client_id_secret(&self) -> Result<(String, String)> {
        let uppercase = self.as_ref().to_uppercase();
        let id = env::var(format!("AUTH_{}_ID", &uppercase))?;
        let secret = env::var(format!("AUTH_{}_SECRET", &uppercase))?;

        Ok((id, secret))
    }

    pub(super) fn oauth_client(&self) -> Result<BasicClient> {
        let (auth_url, token_url) = self.auth_and_token_urls();
        let (client_id, client_secret) = self.client_id_secret()?;

        Ok(oauth2::basic::BasicClient::new(ClientId::new(client_id))
            .set_token_uri(TokenUrl::new(token_url.to_string())?)
            .set_client_secret(ClientSecret::new(client_secret))
            .set_auth_uri(AuthUrl::new(auth_url.to_string())?)
            .set_redirect_uri(RedirectUrl::new(REDIRECT_URL_ENV.to_owned())?))
    }

    pub(super) async fn fetch_user_info(
        &self,
        query_code: &String,
        oauth_client: &BasicClient,
    ) -> Result<String> {
        let http_client = oauth2::reqwest::Client::new();

        let res = match self {
            Self::Strava => {
                let (client_id, client_secret) = self.client_id_secret()?;
                let redirect_uri = REDIRECT_URL_ENV.clone();
                let (_, token_url) = self.auth_and_token_urls();

                http_client
                    .post(token_url)
                    .form(&[
                        ("client_id", client_id.as_str()),
                        ("client_secret", client_secret.as_str()),
                        ("code", query_code),
                        ("grant_type", "authorization_code"),
                        ("redirect_uri", redirect_uri.as_str()),
                    ])
                    .send()
                    .await
                    .context("failed sending strava token request")?
            }

            _ => {
                let token = oauth_client
                    .exchange_code(AuthorizationCode::new(query_code.clone()))
                    .request_async(&http_client)
                    .await?;

                http_client
                    .get(
                        self.get_str("PROFILE_URL")
                            .expect("missing PROFILE_URL for enum"),
                    )
                    .bearer_auth(token.access_token().secret().to_string())
                    .header("User-Agent", "cringe-backend")
                    .send()
                    .await?
            }
        };

        match self {
            Provider::Discord => {
                #[derive(Deserialize)]
                struct ContactInfo {
                    email: Option<String>,
                    verified: bool,
                }

                if let ContactInfo {
                    email: Some(email),
                    verified: true,
                } = res.json::<ContactInfo>().await?
                {
                    return Ok(email);
                }
            }

            Provider::Facebook => {
                #[derive(Deserialize)]
                struct ContactInfo {
                    email: Option<String>,
                }

                if let ContactInfo { email: Some(email) } = res.json::<ContactInfo>().await? {
                    return Ok(email);
                }
            }

            Provider::Google => {
                #[derive(Deserialize)]
                struct ContactInfo {
                    email: String,
                    email_verified: bool,
                }

                if let ContactInfo {
                    email,
                    email_verified: true,
                } = res.json::<ContactInfo>().await?
                {
                    return Ok(email);
                }
            }

            Provider::Github => {
                #[derive(Deserialize)]
                struct ContactInfo {
                    email: String,
                    primary: bool,
                    verified: bool,
                }

                if let Some(primary) = res
                    .json::<Vec<ContactInfo>>()
                    .await?
                    .into_iter()
                    .find(|e| e.primary && e.verified)
                {
                    return Ok(primary.email);
                }
            }

            Provider::Strava => {
                #[derive(Deserialize)]
                struct Athlete {
                    id: i64,
                }

                #[derive(Deserialize)]
                struct ContactInfo {
                    athlete: Athlete,
                }

                let ContactInfo {
                    athlete: Athlete { id },
                } = res.json::<ContactInfo>().await?;

                return Ok(format!("https://www.strava.com/athletes/{id}"));
            }
        };

        Err(anyhow!("no usable contact info found!"))
    }
}
