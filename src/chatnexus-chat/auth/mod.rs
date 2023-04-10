use std::borrow::Cow;
use std::{collections::HashMap, sync::Mutex};

use crate::auth::error::AuthError;
use crate::chat::chatnexus_chat::auth_server::{Auth, AuthServer};
use crate::chat::chatnexus_chat::{AuthRequest, AuthResponse, AuthStage, AuthStatus, AuthType};
use crate::helper;

use mongodb::change_stream::session;
use redis::{AsyncCommands, RedisResult};
use serde::{Deserialize, Serialize};
use urlencoding::encode;

use self::error::AuthResult;

mod auth_grpc;
mod error;

lazy_static::lazy_static! {
    // Helps reduce the amount of calls made to
    // the database.
    static ref PREAUTH_SESSION: Mutex<HashMap<String, AuthStage>> = Mutex::new(HashMap::new());
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AuthSession {
    session_id: String,
    stage: AuthStage,
    url: Option<String>,
    code: Option<String>,
}

#[derive(Clone)]
pub struct AuthService {
    // Authorization Type.
    auth_type: AuthType,
    // Instance of AuthServer.
    pub service: Option<AuthServer<Self>>,
    pub redis: redis::Client,
}

// Defining the implemenation of AuthService
impl AuthService {
    /// Creates a new instance of [AuthService].
    ///
    /// # Arguments
    ///
    /// * `auth_type` - Your authorization type.
    ///
    /// ```
    pub fn new(auth_type: AuthType, redis_cli: redis::Client) -> Self {
        // Initializing the AuthService instance.
        let mut auth_service = Self {
            auth_type,
            service: None,
            redis: redis_cli,
        };
        // Because AuthServer requires an instance of AuthService.
        // we need to mark auth_service as mutable so ->
        // authservice.service can be initialized.
        auth_service.service = Some(AuthServer::new(auth_service.clone()));
        // Returning the AuthService instance.
        auth_service
    }
    /// Catches what [AuthStage] the client is currently on
    /// then executes the appropriate methods..
    ///
    /// # Arguments
    ///
    /// * `current_stage` - The AuthStage of client.
    /// * `target_stage` - What should be executed on this stage.
    /// * `func` - The fn() that will contain the methods.
    ///
    /// ```
    fn catch_stage<T>(&self, current_stage: AuthStage, target_stage: AuthStage, func: T)
    where
        T: FnOnce() -> (),
    {
        if current_stage.eq(&target_stage) {
            func()
        }
    }
    /// Builds an [AuthResponse] for the server. Shorthand
    ///
    /// # Arguments
    ///
    /// * `status` - AuthStatus of the response.
    /// * `stage` - The AuthStage
    /// * `session_id` - The session_id.
    /// * `url` - The OAuth2 url.
    /// * `code` - The code you would like to assign to the user.
    ///
    /// ```
    fn build_response(
        &self,
        status: AuthStatus,
        stage: AuthStage,
        session_id: &str,
        url: Option<String>,
        code: Option<String>,
    ) -> AuthResponse {
        AuthResponse {
            r#type: self.auth_type.into(),
            status: status.into(),
            stage: Some(stage.into()),
            session_id: session_id.into(),
            url: url,
            code: code,
        }
    }

    /// Builds an [AuthSession] then stores the session inside
    /// the redis database.
    ///
    /// # Arguments
    ///
    /// * `session_id` - Session id of client.
    /// * `stage` - The AuthStage (should be Stage2)
    /// * `url` - The OAuth2 url.
    /// * `code` - The authentication code.
    ///
    /// ```
    pub async fn build_session(
        &self,
        stage: AuthStage,
        url: Option<String>,
        code: Option<String>,
    ) -> AuthResult<AuthSession> {
        let conn = &mut self.redis.get_async_connection().await.unwrap();
        let user = AuthSession {
            session_id: helper::gen_uuid(),
            stage,
            url: Some(Self::authorize_link()),
            code,
        };
        let key = format!("session:{}", &user.session_id).to_string();
        let set = conn
            .set(key, serde_json::to_string(&user).unwrap())
            .await
            .map_err(|_| AuthError::SessionNotFound(user.clone().session_id))?;
        Ok(user)
    }

    /// Looks for the user's session id inside redis if
    /// found returns [AuthSession], if not returns [AuthError].
    ///
    /// # Arguments
    ///
    /// * `session_id` - Session id of client.
    ///
    /// ```
    pub async fn get_session(&self, session_id: &str) -> AuthResult<AuthSession> {
        let conn = &mut self.redis.get_async_connection().await.unwrap();
        let key = format!("session:{}", session_id).to_string();
        let session: String = conn
            .get(key)
            .await
            .map_err(|_| AuthError::SessionNotFound(session_id.to_string()))?;
        Ok(serde_json::from_str(&session).unwrap())
    }
    /// Update the AuthStage of a session...
    ///
    /// # Arguments
    ///
    /// * `session_id` - Session id of client.
    /// * `stage` - The field you would like to modify.
    ///
    /// ```
    pub async fn update_stage(&self, session_id: &str, stage: AuthStage) -> AuthResult<()> {
        let mut session = self.get_session(session_id).await?;
        session.stage = stage;
        self.save_session(session_id, session).await?;
        Ok(())
    }
    /// Updates the url of a session.
    ///
    /// # Arguments
    ///
    /// * `session_id` - Session id of client.
    /// * `stage` - The field you would like to modify.
    ///
    /// ```
    pub async fn update_url(&self, session_id: &str, url: &Option<String>) -> AuthResult<()> {
        if url.is_some() && !url.clone().unwrap().is_empty() {
            let mut session = self.get_session(session_id).await?;
            session.url = url.clone();
            self.save_session(session_id, session).await?;
        }
        Ok(())
    }
    /// Updates the code of a session.
    ///
    /// # Arguments
    ///
    /// * `session_id` - Session id of client.
    /// * `stage` - The field you would like to modify.
    ///
    /// ```
    pub async fn update_code(&self, session_id: &str, code: &Option<String>) -> AuthResult<()> {
        if code.is_some() && !code.clone().unwrap().is_empty() {
            let mut session = self.get_session(session_id).await?;
            session.code = code.clone();
            self.save_session(session_id, session).await?;
        }
        Ok(())
    }
    /// Purpose is to save sessions that already exist
    ///
    /// # Arguments
    ///
    /// * `session_id` - Session id of client.
    /// * `session` - The session that was retrieved.
    ///
    /// ```
    async fn save_session(&self, session_id: &str, session: AuthSession) -> AuthResult<()> {
        let conn = &mut self.redis.get_async_connection().await.unwrap();
        let key = format!("session:{}", session_id).to_string();
        conn.set(key, serde_json::to_string(&session).unwrap())
            .await
            .map_err(|_| AuthError::FailedToUpdateSession(session_id.to_string()))
    }

    /// Creates an OAuth2 URL
    fn authorize_link() -> String {
        format!(
            "{}?response_type=code&client_id={}&scope={}&redirect_uri={}&prompt=consent",
            dotenv::var("OAUTH2_AUTHORIZE").unwrap(),
            dotenv::var("OAUTH2_CLIENT_ID").unwrap(),
            encode(&dotenv::var("OAUTH2_SCOPES").unwrap()),
            encode(&dotenv::var("OAUTH2_REDIRECT_URI").unwrap())
        )
        .to_string()
    }

    /// Returns instance of [AuthServer].
    pub fn service(self) -> AuthServer<AuthService> {
        self.service.unwrap()
    }
}
