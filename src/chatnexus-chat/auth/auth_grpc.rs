use tonic::{Request, Response, Status};

use crate::{chat::{chatnexus_chat::{auth_server::{Auth}, AuthResponse, AuthRequest, AuthStage, AuthStatus}}, auth::PREAUTH_SESSION, helper};

use super::AuthService;

#[tonic::async_trait]
impl Auth for AuthService {
    async fn notify_auth_service(
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
            let mut response = self.build_response(
                AuthStatus::Denied, 
                AuthStage::Stage1, 
                session_id, 
                None, 
                None
            );
            self.catch_stage(auth_stage, AuthStage::Stage1, || {
                response.stage = Some(AuthStage::Stage2.into());
                println!("moving to stage two");
            });

            self.catch_stage(auth_stage, AuthStage::Stage2, || {
                // generate info they need..
                response.stage = Some(AuthStage::Stage3.into());
                println!("moving to stage three");
            });

            self.catch_stage(auth_stage, AuthStage::Stage3, || {
                response.stage = Some(AuthStage::Stage3.into());
                println!("WOAH FINAL STAGE!!!");
                // Wait for them to finish logging in. continous requests should be sent until the user has logged in
            });
            sessions.insert(session_id.to_string(), AuthStage::from_i32(response.stage.unwrap()).unwrap());
            return Ok(Response::new(response))
        } else {
            // Notifying the server we recieved an Authentication request.
            helper::system_print("Building a new Authentication request.");
            // If the session ID is not valid, generate a valid session ID.
            let session_id = uuid::Uuid::new_v4().simple().to_string();
            // Building the gRPC response.
            let response = self.build_response(AuthStatus::Ok, AuthStage::Stage1, &session_id, None, None);
            // Insert the newly generated session ID into PREAUTH_SESSIONS 
            // with the initial authentication stage.
            sessions.insert(session_id, AuthStage::Stage1);
            // Sending out the gRPC response.
            return Ok(Response::new(response));
        }
        todo!()
    }
}

        //let session_id = data.session_id();
           // sessions.insert(session_id.to_string(), AuthStage::Stage2);
            //let response = self.build_response(AuthStatus::Ok, AuthStage::Stage2, session_id, None, None);
            //wself.catch_stage(sessions.get(k), target_stage, ());

  //self.catch_stage(current_stage, target_stage, func)
        /*
        &self.stage_change(request, Stage::1
            ({
                /// boom do 
            })
        );

        &self.stage_change(request, Stage::2
            ({
                /// boom do 
            })
        );
        */