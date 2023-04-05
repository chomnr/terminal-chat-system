use std::io::{self, Write, stdout, stdin};

use chatnexus_chat::{chat_client::ChatClient, AuthStatus, AuthStage};
use oauth2::http::{request, Request};

use crate::chatnexus_chat::{ChatRequest, auth_client::AuthClient, Empty, AuthRequest, AuthType};

pub mod chatnexus_chat {
    tonic::include_proto!("chatnexus.chat");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // User Information.
    let mut session_id: Option<String> = None;
    let mut auth_stage: Option<AuthStage> = None;
    let mut auth_status: Option<AuthStatus> = None;

    let address = "http://[::1]:50051";

    let mut chat_client = ChatClient::connect(address)
        .await
        .unwrap();
    let mut auth_client = AuthClient::connect(address)
        .await
        .unwrap();

    //let request = tonic::Request::new(AuthRequest {
        //session_id: None
    //});
    
    while (true) {
        //let response = auth_client.notify_auth_service(todo!()).await?;
        //session_id = Some(response.get_ref().session_id.clone());
        //auth_stage = Some(response.get_ref().stage());

        
    }

    //let response = auth_client.notify_auth_service(request).await?;

    //let session_id = &response.get_ref().session_id;
    //let auth_type = AuthType::from_i32(response.get_ref().r#type).unwrap();
    //let auth_stage = AuthStage::from_i32(response.get_ref().stage.unwrap()).unwrap();

    //let method: AuthType = AuthType::from_i32(response.get_ref().r#type).unwrap();
    //println!("Authentication Method: {:?}\n", method);
    
    //println!("Response: {:?}", response);
    //stdout().flush().unwrap(); // flush the output to the console
    //let mut answer = String::new();
    //stdin().read_line(&mut answer).unwrap();
    Ok(())
}

   // note: when a user responds yes to start authentication (they send a request that they're ready)
   // the server generates an auth_session_id and sends it back to them and that will be used to identify them.
   // A URL WILL BE SENT AS WELL AND THE AUTHENTICATION CODE THAT IS DIRECTLY LINKED TO THE CLIENT...
   // after 2 minutes the session will be deleted if the user does not authorize it....