use std::net::SocketAddr;

use tonic::{Response, Request, Status};
use tonic::transport::Server;

use crate::auth::auth::chatnexus_auth::auth_server;
use  crate::auth::auth::chatnexus_auth::{AuthRequest, AuthResponse};

use self::chatnexus_auth::AuthType;
use self::chatnexus_auth::auth_server::{Auth, AuthServer};


pub mod chatnexus_auth {
    tonic::include_proto!("chatnexus.auth");
}

#[derive(Default, Clone)]
pub struct ChatNexusAuth;

#[tonic::async_trait]
impl Auth for ChatNexusAuth {
    async fn set_auth_type(&self, request: Request<AuthRequest>) -> Result<Response<AuthResponse>, Status>  {
        println!("test");
        todo!()
    }
}

pub struct AuthService {
    method: AuthType,
    auth: ChatNexusAuth,
    service: AuthServer<ChatNexusAuth>,
}

impl AuthService {
    pub fn new(method: AuthType) -> Self {
        let auth = ChatNexusAuth::default();
        let service = auth_server::AuthServer::new(auth.clone());
        Self {
            method: method,
            auth,
            service,
        }
    }

    pub fn service(self) -> AuthServer<ChatNexusAuth> {
        self.service
    }
}