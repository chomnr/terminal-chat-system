use std::io::{self, Write, stdout, stdin};

use chatnexus_chat::{chat_client::ChatClient, AuthStatus, AuthStage};

use crate::chatnexus_chat::{ChatRequest, auth_client::AuthClient, Empty, AuthRequest, AuthType};

pub mod chatnexus_chat {
    tonic::include_proto!("chatnexus.chat");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "http://[::1]:50051";

    let mut chat_client = ChatClient::connect(address)
        .await
        .unwrap();
    let mut auth_client = AuthClient::connect(address)
        .await
        .unwrap();

    let request = tonic::Request::new(ChatRequest {
        username: "Harry".to_string(),
        message: "Testing this".to_string()
    });

    let request = tonic::Request::new(AuthRequest {
        session_id: None
    });

    let response = auth_client.notify_auth_service(request).await?;

    let method: AuthType = AuthType::from_i32(response.get_ref().r#type).unwrap();
    println!("Authentication Method: {:?}\n", method);
    
    //println!("Response: {:?}", response);
    stdout().flush().unwrap(); // flush the output to the console
                let mut answer = String::new();
                stdin().read_line(&mut answer).unwrap();
    Ok(())
}

   // note: when a user responds yes to start authentication (they send a request that they're ready)
   // the server generates an auth_session_id and sends it back to them and that will be used to identify them.
   // A URL WILL BE SENT AS WELL AND THE AUTHENTICATION CODE THAT IS DIRECTLY LINKED TO THE CLIENT...
   // after 2 minutes the session will be deleted if the user does not authorize it....