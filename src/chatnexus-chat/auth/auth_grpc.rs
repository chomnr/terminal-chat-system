use dialoguer::console::Term;
use tonic::{Request, Response, Status};

use crate::{
    chat::ChatUser,
    chatnexus_chat::{
        auth_server::Auth, AuthPresenseResponse, AuthRequest, AuthResponse, AuthStage, AuthStatus,
        AuthVerifyRequest, AuthVerifyResponse, Empty,
    },
    helper::{self},
};

use super::AuthService;

#[tonic::async_trait]
impl Auth for AuthService {
    async fn notify_presence(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<AuthPresenseResponse>, Status> {
        Ok(Response::new(AuthPresenseResponse {
            auth_type: self.auth_type.into(),
        }))
    }

    async fn promote_stage(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        match self.get_session(&request.get_ref().session_id).await {
            Ok(session) => {
                let mut response = self.build_response(
                    AuthStatus::Ok,
                    session.stage,
                    &session.session_id,
                    session.url,
                    session.code.clone(),
                );
                self.catch_stage(session.stage, AuthStage::Prerequisites, || {
                    response.code = Some(helper::gen_string(7));
                    response.set_stage(AuthStage::Authorization);
                });
                self.catch_stage(session.stage, AuthStage::Authorization, || {
                    //response.set_stage(AuthStage::Authorization);
                    //response.code = Some(helper::gen_string(7));
                });
                self.catch_stage(session.stage, AuthStage::Completed, || {
                    //response.set_stage(AuthStage::Completed);
                });
                self.update_stage(
                    &session.session_id,
                    AuthStage::from_i32(response.stage.unwrap()).unwrap(),
                )
                .await
                .unwrap();
                if session.stage != response.stage() {
                    self.update_stage(&session.session_id, response.stage())
                        .await
                        .unwrap();
                }
                if session.code.is_none() {
                    self.update_code(&session.session_id, &response.code)
                        .await
                        .unwrap();
                }
                return Ok(Response::new(response));
            }
            Err(_) => {
                let newly_created = self
                    .build_session(AuthStage::Prerequisites, None, None)
                    .await
                    .unwrap();
                return Ok(Response::new(self.build_response(
                    AuthStatus::Ok,
                    AuthStage::Prerequisites,
                    &newly_created.session_id,
                    newly_created.url,
                    None,
                )));
            }
        }
    }

    async fn check_auth_stage(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let data = request.get_ref();
        let future = self.get_session(&data.session_id).await;
        let response = match future {
            Ok(res) => self.build_response(AuthStatus::Ok, res.stage, &res.session_id, None, None),
            Err(_) => self.build_response(AuthStatus::Denied, AuthStage::Prerequisites, "", None, None),
        };
        Ok(Response::new(response))
    }

    async fn verify_user(
        &self,
        request: tonic::Request<AuthVerifyRequest>,
    ) -> Result<Response<AuthVerifyResponse>, Status> {
        let data = request.get_ref();
        if data.secret_key.eq(&dotenv::var("WEB_SECRET_KEY").unwrap()) {
            println!("Request {:?}", data);
            let user_info = ChatUser {
                uid: data.uid.to_string(),
                username: data.username.to_string(),
                discriminator: data.discriminator.to_string(),
                session_id: data.session_id.to_string(),
            };
        }
        todo!()
    }
}