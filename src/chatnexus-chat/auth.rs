use std::default;

use tonic::{Request, Response, Status};

use crate::chat::{chatnexus_chat::{AuthType, auth_server::{self, Auth, AuthServer}, self, Empty, BarenResponse, chat_server::ChatServer}, ChatNexusChat};



#[derive(Clone)]
pub struct AuthService {
    auth_type: AuthType,
    service: Option<AuthServer<Self>>
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn send_auth_message(&self, _request: Request<Empty>) -> Result<Response<BarenResponse>, Status> {
        println!("[SYSTEM] Sending out an authentication request.");
        if !self.auth_type.eq(&AuthType::None) {
            let response = BarenResponse {
                message: "".to_string()
            };
        }
        let response = BarenResponse {
            message: format!("{:?}", self.auth_type)
        };
        Ok(Response::new(response))
    }
}

impl AuthService {
    pub fn new(auth_type: AuthType) -> Self {
       // let service = AuthServer::new(auth.clone());
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