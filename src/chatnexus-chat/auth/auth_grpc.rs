
use dialoguer::console::Term;
use tonic::{Request, Response, Status};

use crate::{
    auth::PREAUTH_SESSION,
    chat::chatnexus_chat::{auth_server::Auth, AuthRequest, AuthResponse, AuthStage, AuthStatus, AuthPresenseResponse, Empty},
    helper::{self, gen_string},
};

use super::{AuthService, error::AuthError};

#[tonic::async_trait]
impl Auth for AuthService {
    async fn notify_presence(&self, request: Request<Empty>) -> Result<Response<AuthPresenseResponse>, Status> {
        Ok(Response::new(AuthPresenseResponse {
            auth_type: self.auth_type.into(),
        }))
    }
    async fn promote_stage(&self, request: Request<AuthRequest>) -> Result<Response<AuthResponse>, Status> {
        match self.get_session(&request.get_ref().session_id).await {
            Ok(session) => {
                let mut response = self.build_response(
                    AuthStatus::Ok,
                    AuthStage::Stage1,
                    &session.session_id,
                    session.url, session.code);
                self.catch_stage(session.stage, AuthStage::Stage1, || {
                    response.set_stage(AuthStage::Stage2);
                });
                self.catch_stage(session.stage, AuthStage::Stage2, || {
                    response.set_stage(AuthStage::Stage3);
                    response.url = Some("Hello.com".to_string());
                    response.code = Some(helper::gen_string(7));
                });
                self.catch_stage(session.stage, AuthStage::Stage3, || {
                    response.set_stage(AuthStage::Stage3)
                });
                self.update_stage(
                    &session.session_id, 
                    AuthStage::from_i32(response.stage.unwrap()).unwrap()
                ).await.unwrap();
                self.update_url(&session.session_id, &response.url).await.unwrap();
                self.update_code(&session.session_id, &response.code).await.unwrap();
                return Ok(Response::new(response))
            },
            Err(_) => {
                let newly_created = self.build_session(AuthStage::Stage1, None, None).await.unwrap();
                return Ok(Response::new(self.build_response(AuthStatus::Ok, AuthStage::Stage1, &newly_created.session_id, None, None)))
            },
        }
    }
    /*
    async fn check_auth_stage(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let data = request.get_ref();
        let future = self.get_session(&data.session_id).await;
        let response = match future {
            Ok(res) => self.build_response(AuthStatus::Ok, res.stage, &res.session_id, None, None),
            Err(_) => self.build_response(AuthStatus::Denied, AuthStage::Stage1, "", None, None),
        };
        Ok(Response::new(response))
    }
     */
}


/*


let pre_sess = self.get_session(&request.get_ref().session_id).await;
        if pre_sess.is_ok() {
            let session = pre_sess.unwrap();
            let (c_stage, mut t_stage) = (session.stage, AuthStage::from_i32(session.stage as i32 + 1));
            
            todo!();
        } else {
            
        }

 */
 //let future = self.get_session(&data.session_id).await;

        /*

        let session_id = uuid::Uuid::new_v4().simple().to_string();
            let response = self.build_response(AuthStatus::Ok, AuthStage::Stage1, &session_id, None, None);
            self.build_session(&session_id, AuthStage::Stage1, None, None).await;
            return Ok(Response::new(response));


        if future.is_ok() {
            let session = future.unwrap();
            let session_id = session.session_id;
            let current_stage = session.stage;
            let mut response: AuthResponse = self.build_response(AuthStatus::Ok, AuthStage::Stage1, &session_id, None, None);
            self.catch_stage(current_stage, AuthStage::Stage1, || {
                response.stage = Some(AuthStage::Stage2.into());
            });
            self.catch_stage(current_stage, AuthStage::Stage2, || {
                response.stage = Some(AuthStage::Stage3.into());
                println!("STAGE 2")
            });
            self.catch_stage(current_stage, AuthStage::Stage3, || {
                response.stage = Some(AuthStage::Stage3.into());
                println!("STAGE 3")
            });
            self.update_stage(&session_id, AuthStage::).await.unwrap();
            Ok(Response::new(response))
        } else {
            // If the session ID is not valid, generate a valid session ID.
            let session_id = uuid::Uuid::new_v4().simple().to_string();
            // Notifying the server we recieved an Authentication request.
            helper::system_print(
                &format!("Starting an authentication for '{}'.", session_id).to_string(),
            );
            // Building the gRPC response.
            let response = self.build_response(AuthStatus::Ok, AuthStage::Stage1, &session_id, None, None);
            // Building the session.
            self.build_session(&session_id, AuthStage::Stage1, None, None).await;
            // Sending out the gRPC response.
            return Ok(Response::new(response));
        }
        */