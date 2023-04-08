use tonic::{Request, Response, Status};

use crate::{chat::{chatnexus_chat::{auth_server::{Auth}, AuthResponse, AuthRequest, AuthStage, AuthStatus}}, auth::PREAUTH_SESSION, helper::{self, gen_string}};

use super::AuthService;

#[tonic::async_trait]
impl Auth for AuthService {
    async fn notify_auth2_service(
        &self, 
        request: Request<AuthRequest>
    ) -> Result<Response<AuthResponse>, Status> {
        // Extract the AuthRequest RPC from the incoming request.
        let data = request.get_ref();
        // Obtain a reference to the PREAUTH_SESSION global HashMap.
        let mut sessions = PREAUTH_SESSION.lock().unwrap();
        // Check if the session ID extracted from the request is valid.
        if sessions.contains_key(data.session_id()) {
            let session_id = data.session_id();
            let auth_stage: AuthStage = *sessions.get(session_id).unwrap();
            // gRPC Response
            let mut response = self.build_response(AuthStatus::Denied, AuthStage::Stage1, session_id, None, None);
            // Stage 1
            self.catch_stage(auth_stage, AuthStage::Stage1, || {
                response.stage = Some(AuthStage::Stage2.into());
                response.status  = AuthStatus::Ok.into();
                //helper::system_print(&format!("Authorizing '{}' for Stage 2.", session_id).to_string())
            });
            // Stage 2
            self.catch_stage(auth_stage, AuthStage::Stage2, || {
                response.stage = Some(AuthStage::Stage3.into());
                response.status  = AuthStatus::Ok.into();
                helper::system_print(&format!("Authorizing '{}' for Stage 3.", session_id).to_string())
            });
            // Stage 3
            self.catch_stage(auth_stage, AuthStage::Stage3, || {
                response.stage = Some(AuthStage::Stage3.into());
                response.status  = AuthStatus::Ok.into();
                helper::system_print(&format!("'{}' has been fully authenticated.", session_id).to_string())
            });
            sessions.insert(session_id.to_string(), AuthStage::from_i32(response.stage.unwrap()).unwrap());
            return Ok(Response::new(response))
        } else {
            // If the session ID is not valid, generate a valid session ID.
            let session_id = uuid::Uuid::new_v4().simple().to_string();
            // Notifying the server we recieved an Authentication request.
            helper::system_print(&format!("Starting an authentication for '{}'.", session_id).to_string());
            // Building the gRPC response.
            let response = self.build_response(AuthStatus::Ok, AuthStage::Stage1, &session_id, None, None);
            // Insert the newly generated session ID into PREAUTH_SESSIONS 
            // with the initial authentication stage.
            sessions.insert(session_id, AuthStage::Stage1);
            // Sending out the gRPC response.
            return Ok(Response::new(response));
        }
    }
}

/*

 // Stage 1
            self.catch_stage(auth_stage, AuthStage::Stage1, || {
                response.stage = Some(AuthStage::Stage2.into());
                response.status  = AuthStatus::Ok.into();
                helper::system_print(&format!("Authorizing '{}' for Stage 2.", session_id).to_string())
            });
            // Stage 2
            self.catch_stage(auth_stage, AuthStage::Stage2, || {
                // generate info they need..
                response.stage = Some(AuthStage::Stage3.into());
                response.status  = AuthStatus::Ok.into();
                helper::system_print(&format!("Authorizing '{}' for Stage 3.", session_id).to_string())
            });
            // Stage 3
            self.catch_stage(auth_stage, AuthStage::Stage3, || {
                response.stage = Some(AuthStage::Stage3.into());
                response.status  = AuthStatus::Ok.into();
                helper::system_print(&format!("'{}' has been fully authenticated.", session_id).to_string())
                // Wait for them to finish logging in. continous requests should be sent until the user has logged in
            });
 */