use std::{borrow::Borrow, fmt::format};

use oauth2::{ClientId, ClientSecret, AuthUrl, TokenUrl, RedirectUrl, basic::{BasicClient, BasicTokenType}, AuthorizationCode, StandardTokenIntrospectionResponse, ExtraTokenFields, TokenType, EmptyExtraTokenFields, StandardTokenResponse, AccessToken, http::HeaderValue};
use rocket::config;

use serde::{Serialize, Deserialize};
use urlencoding::encode;

// Simplify the oauth2::basic::BasicClient
pub struct OAuth2 {
    config: OAuth2Config,
    pub client: BasicClient
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OAuth2Config {
    client_id: ClientId,
    client_secret: Option<ClientSecret>,
    auth_url: AuthUrl,
    token_url: Option<TokenUrl>,
    redirect_uri: RedirectUrl
}

// Services
#[derive(Serialize, Deserialize)]
pub struct DiscordUser {
    id: String,
    username: String,
    discriminator: String,
    avatar: String
}

impl Default for OAuth2Config {
    fn default() -> Self {
        Self { 
            client_id: ClientId::new(dotenv::var("OAUTH2_CLIENT_ID").unwrap()), 
            client_secret: Some(ClientSecret::new(dotenv::var("OAUTH2_CLIENT_SECRET").unwrap())), 
            auth_url: AuthUrl::new(dotenv::var("OAUTH2_AUTHORIZE").unwrap()).unwrap(), 
            token_url: Some(TokenUrl::new(dotenv::var("OAUTH2_TOKEN").unwrap()).unwrap()), 
            redirect_uri: RedirectUrl::new(dotenv::var("OAUTH2_REDIRECT_URI").unwrap()).unwrap() 
        }
    }
}


impl OAuth2Config {
    pub fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    pub fn client_secret(&self) -> &Option<ClientSecret> {
        &self.client_secret
    }

    pub fn auth_url(&self) -> &AuthUrl {
        &self.auth_url
    }

    pub fn token_url(&self) -> &Option<TokenUrl> {
        &self.token_url
    }

    pub fn redirect_uri(&self) -> &RedirectUrl {
        &self.redirect_uri
    }
}

impl OAuth2 {
    /// Simplified version of BasicClient
    ///
    /// # Arguments
    ///
    /// * `config` - The OAuth2Config.
    ///
    /// ```
    pub fn new(config: OAuth2Config) -> Self {
        // hacky way but works..
        let cloned_config = config.clone();
        let client = BasicClient::new(
            config.client_id, 
            config.client_secret, 
            config.auth_url, 
            config.token_url
        ).set_redirect_uri(RedirectUrl::new(cloned_config.redirect_uri().to_string()).unwrap());
        Self {
            config: cloned_config,
            client,
        }
    }
    /// Exchange the authorization code for access_code.
    ///
    /// # Arguments
    ///
    /// * `code` - The authorization code.
    ///
    /// ```
    pub async fn exchange_auth_code(&self, code: String) -> StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>  {
        self.client.exchange_code(AuthorizationCode::new(code)).request_async(oauth2::reqwest::async_http_client).await.unwrap()
    }
    /// Returns Discord User Struct
    ///
    /// # Arguments
    ///
    /// * `access_token` - The access_token.
    ///
    /// ```
    pub async fn post_discord(&self, access_token: &AccessToken) -> DiscordUser {
        let client = reqwest::Client::new();
        let response = client
            .get("https://discord.com/api/v10/users/@me")
            .header("content-type","application/x-www-form-urlencoded")
            .bearer_auth(access_token.secret()).send().await.unwrap();
         let result: DiscordUser = serde_json::from_str(&response.text().await.unwrap()).unwrap();
         result
    }
    /// Returns the Authorization URL
    pub fn authorize_url(&self) -> String {
        format!(
            "{}?response_type=code&client_id={}&scope={}&redirect_uri={}&prompt=consent",
            self.config.auth_url().to_string(),
            self.config.client_id().to_string(),
            encode(&dotenv::var("OAUTH2_SCOPES").unwrap()),
            encode(&self.config.redirect_uri().to_string())
        )
    }
}

impl DiscordUser {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn discriminator(&self) -> &str {
        &self.discriminator
    }
    pub fn avatar(&self) -> &str {
        &self.avatar
    }
}