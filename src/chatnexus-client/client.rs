use std::{io::{self, Write, stdout, stdin}};

use chatnexus_chat::{chat_client::ChatClient, AuthStatus, AuthStage};
use dialoguer::{Confirm, theme::{SimpleTheme, self, ColorfulTheme}};
use oauth2::http::{request, Request};

use crate::chatnexus_chat::{ChatRequest, auth_client::AuthClient, Empty, AuthRequest, AuthType};

pub mod chatnexus_chat {
    tonic::include_proto!("chatnexus.chat");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // User Information.
    let mut session_id = String::default();
    let mut auth_stage: Option<AuthStage> = None;
    let mut auth_status: Option<AuthStatus> = None;
    let address = "http://[::1]:50051";
    let mut chat_client = ChatClient::connect(address)
        .await
        .unwrap();
    let mut auth_client = AuthClient::connect(address)
        .await
        .unwrap();

    while (true) {
        if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to proceed?").interact().unwrap() {
            println!("Executing request...");
            let request = tonic::Request::new(AuthRequest {
                session_id: Some(session_id.to_string())
            });
            if session_id.is_empty() {
                println!("CREATING_SESSION_ID");
                let response = auth_client.notify_auth_service(request).await?;
                session_id = response.get_ref().session_id.to_string();
            } else {
                println!("EXISTING_SESSION_ID");
                let response = auth_client.notify_auth_service(request).await?;
                println!("{:?}", response)
            }
        } else {
            println!("nevermind then :(");
        }
    }
    Ok(())
}

   // note: when a user responds yes to start authentication (they send a request that they're ready)
   // the server generates an auth_session_id and sends it back to them and that will be used to identify them.
   // A URL WILL BE SENT AS WELL AND THE AUTHENTICATION CODE THAT IS DIRECTLY LINKED TO THE CLIENT...
   // after 2 minutes the session will be deleted if the user does not authorize it....