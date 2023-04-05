use std::{default, collections::HashMap};

use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

use crate::{chat::{chatnexus_chat::{AuthType, auth_server::{self, Auth, AuthServer}, self, Empty, BarenResponse, chat_server::ChatServer, AuthStage}, ChatNexusChat}, helper};

#[derive(Clone)]
pub struct AuthService {
    auth_type: AuthType,
    service: Option<AuthServer<Self>>
}

lazy_static::lazy_static! {
    // After Auth::Stage_1 this is no longer used...
    static ref PREAUTH_SESSION: Mutex<HashMap<String, AuthStage>> = Mutex::new(HashMap::new());
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn notify_auth_service(&self, request: Request<Empty>) -> Result<Response<BarenResponse>, Status> {
        let data = request.get_ref();
        /* 
        let req_data = request.get_ref();
        let mut message = "Sent out an authentication request.";
        if !req_data.ip.is_none() {
        }

        let response = BarenResponse {
            message: format!("{:?}", self.auth_type)
        };
        */
       /// helper::system_print(format!("Sent out an authentication request.").as_str());
       //// helper::system_print(format!("Waiting for response from {:?}.", req_data.ip.unwrap().to_string()).as_str());
       // let response = BarenResponse {
         ///   message: format!("{:?}", self.auth_type)
        //};
        //Ok(Response::new(todo!()))
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