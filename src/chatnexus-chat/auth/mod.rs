use std::{collections::HashMap, sync::Mutex};

use crate::chat::chatnexus_chat::{AuthStage, AuthType, AuthStatus, AuthResponse};
use crate::chat::chatnexus_chat::auth_server::{AuthServer, Auth};

mod auth_grpc;

lazy_static::lazy_static! {
    // Helps reduce the amount of calls made to
    // the database.
    static ref PREAUTH_SESSION: Mutex<HashMap<String, AuthStage>> = Mutex::new(HashMap::new());
}

#[derive(Clone)]
pub struct AuthService {
    auth_type: AuthType,
    service: Option<AuthServer<Self>>
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
        // Because Tonic (gRPC) requires an instance of AuthService.
        // we need to mark auth_service as mutable so ->
        // authservice.service can be initialized.
        auth_service.service = Some(AuthServer::new(auth_service.clone()));
        // Returning the AuthService instance.
        auth_service
    }

    fn build_response(self, status: AuthStatus, stage: AuthStage, session_id: &str) -> AuthResponse  {
        AuthResponse { 
            r#type: self.auth_type.into(), 
            status: status.into(), 
            stage: Some(stage.into()), 
            session_id: session_id.into()
        }
    }

    pub fn service(self) -> AuthServer<AuthService> {
        self.service.unwrap()
    }
}
