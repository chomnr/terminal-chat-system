use std::{default, collections::HashMap};

use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

use crate::{chat::{chatnexus_chat::{AuthType, auth_server::{self, Auth, AuthServer}, self, Empty, BarenResponse, chat_server::ChatServer, AuthStage, AuthResponse, AuthRequest, AuthStatus}, ChatNexusChat}, helper};

lazy_static::lazy_static! {
    // After Auth::Stage_1 this is no longer used...
    // only used so we dont have to make unnecessary
    // calls to the database(redis and or mongodb).
    static ref PREAUTH_SESSION: Mutex<HashMap<String, AuthStage>> = Mutex::new(HashMap::new());
}

#[derive(Clone)]
pub struct AuthService {
    auth_type: AuthType,
    service: Option<AuthServer<Self>>
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn notify_auth_service(&self, request: Request<AuthRequest>) -> Result<Response<AuthResponse>, Status> {
        let data = request.get_ref();
        let mut auth_session = PREAUTH_SESSION.lock().await;
        /*
        if data.session_id.is_none() {
            helper::system_print("Sent out an authentication request.");
            let session_id = uuid::Uuid::new_v4().as_simple().to_string();
            auth_session.insert(session_id.clone(), AuthStage::Stage1);
            let response = AuthResponse {
                session_id: session_id.into(),
                stage: Some(AuthStage::Stage1.into()),
                status: AuthStatus::Ok.into(),
                r#type: self.auth_type.into()
            };
            return Ok(Response::new(response))
        } else {
            let session_id = request.get_ref().session_id.clone().unwrap();
            let status = AuthStatus::Pending;
            if auth_session.contains_key(&session_id) {
                //check for the
            }
            let response = AuthResponse {
                session_id: session_id.into(),
                stage: None,
                status: status.into(),
                r#type: self.auth_type.into()
            };
            return Ok(Response::new(response))
        }
        */
        todo!()
    }
}

impl AuthService {
    pub fn new(auth_type: AuthType) -> Self {
        Self {
            auth_type,
            service: None,
        }
    }

    // hacky way until i find a better solution...
    pub fn reset_service(&mut self) {
        self.service = Some(AuthServer::new(self.clone()));
    }
    
    pub fn service(self) -> AuthServer<AuthService> {
        self.service.unwrap()
    }
}