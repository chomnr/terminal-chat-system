use std::{collections::HashMap, sync::Mutex};

use crate::chat::chatnexus_chat::{AuthStage, AuthType, AuthStatus, AuthResponse, AuthRequest};
use crate::chat::chatnexus_chat::auth_server::{AuthServer, Auth};

mod auth_grpc;

lazy_static::lazy_static! {
    // Helps reduce the amount of calls made to
    // the database.
    static ref PREAUTH_SESSION: Mutex<HashMap<String, AuthStage>> = Mutex::new(HashMap::new());
}

#[derive(Clone)]
pub struct AuthService {
    // Authorization Type.
    auth_type: AuthType,
    // Instance of AuthServer.
    pub service: Option<AuthServer<Self>>
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
    pub fn new(auth_type: AuthType) -> Self {
        // Initializing the AuthService instance.
        let mut auth_service = Self {
            auth_type,
            service: None,
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
    fn catch_stage(&self, 
        current_stage: AuthStage, 
        target_stage: AuthStage, 
        func: fn()) {
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
    /// 
    /// ```
    fn build_response(&self, 
        status: AuthStatus, 
        stage: AuthStage, 
        session_id: &str
    ) -> AuthResponse  {
        AuthResponse { 
            r#type: self.auth_type.into(), 
            status: status.into(), 
            stage: Some(stage.into()), 
            session_id: session_id.into()
        }
    }
    /// Builds an [AuthRequest] for the client. Shorthand
    /// 
    /// # Arguments
    /// 
    /// * `session_id` - The session_id of the client.
    /// 
    /// ```
    pub fn build_request(session_id: &str) -> AuthRequest {
        AuthRequest { 
            session_id: Some(session_id.into())
        }
    }
    /// Returns instance of [AuthServer].
    pub fn service(self) -> AuthServer<AuthService> {
        self.service.unwrap()
    }
}
