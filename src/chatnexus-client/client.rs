use std::io::{self, stdin, stdout, Write};

use chatnexus_chat::{chat_client::ChatClient, AuthStage, AuthStatus};
use dialoguer::{
    console::Term,
    theme::{self, ColorfulTheme, SimpleTheme},
    Confirm,
};
use mongodb::change_stream::session;
use oauth2::{
    helpers,
    http::{request, Request},
};

use crate::chatnexus_chat::{auth_client::AuthClient, AuthRequest, AuthType, ChatRequest, Empty};

pub mod chatnexus_chat {
    tonic::include_proto!("chatnexus.chat");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("    _____ _           _   _   _                     ");
    println!(r"  / ____| |         | | | \ | |                    ");
    println!(r" | |    | |__   __ _| |_|  \| | _____  ___   _ ___ ");
    println!(r" | |    | '_ \ / _` | __| . ` |/ _ \ \/ / | | / __|");
    println!(r" | |____| | | | (_| | |_| |\  |  __/>  <| |_| \__ \");
    println!(r"  \_____|_| |_|\__,_|\__|_| \_|\___/_/\_\\__,_|___/");
    println!("");

    // Address of the server we would like to connect to.
    let address = "http://[::1]:50051";
    // Connecting to AuthService
    let mut auth_client = AuthClient::connect(address).await.unwrap();
    // Client's Session ID
    let mut session_id = String::default();
    loop {
        // [Request/Response] Presence gRPC
        let notify_presence = auth_client.notify_presence(Empty::default()).await?;
        let presence_result = notify_presence.get_ref();

        // [Request] Stage Promotion gRPC
        let mut request = AuthRequest { session_id: session_id.clone() };

        println!("Server Authorization Method: {:?}\n", AuthType::from_i32(presence_result.auth_type).unwrap());
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Begin Authorization?")
            .interact()
            .unwrap()
        {   
            // [Response] Stage Promotion gRPC
            let auth_promote = auth_client.promote_stage(request).await.unwrap();
            let promote_result = auth_promote.get_ref();
            println!("{:?}", promote_result);
        }
    }
    Ok(())
}

/*
    // Mutable Request/Response
    let mut request = AuthRequest {session_id: session_id.to_string()};
    let mut response = auth_client.notify_auth_service(request).await?;
    let mut result = response.get_ref();

    // Result References
    let a_type = AuthType::from_i32(result.r#type).unwrap();
    let stage = AuthStage::from_i32(result.stage.unwrap()).unwrap();
    let status = AuthStatus::from_i32(result.status).unwrap();

    println!("Server Authorization Method: {:?}\n", a_type);
    //println!("Session ID: {:?}\n", session_id);

    if stage == AuthStage::Stage1 && status == AuthStatus::Ok && session_id.is_empty() {
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to proceed?")
            .interact()
            .unwrap()
        {
            println!("{:?}", response);
            session_id = result.session_id.to_string();
        }
    }

    if stage == AuthStage::Stage2 && status == AuthStatus::Ok && session_id.is_empty() {
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Hello.")
            .interact()
            .unwrap()
        {
            println!("{:?}", response);
            session_id = result.session_id.to_string();
        }
    }
}
*/
/*

  if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to proceed?")
        .interact()
        .unwrap()
    {

    }
if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to proceed?")
        .interact()
        .unwrap()
    {
        let request = AuthRequest {
            session_id: Some(session_id.to_string())
        };
        if session_id.is_empty() {
            let response = auth_client.notify_auth2_service(request).await?;
            session_id = response.get_ref().session_id.to_string()
        } else {
            let response = auth_client.notify_auth2_service(request).await?;
            println!("{:?}", response);
        }
    }
// User Information.
//let mut session_id = String::default();
//let mut auth_stage: Option<AuthStage> = None;
//let mut auth_status: Option<AuthStatus> = None;
//let address = "http://[::1]:50051";
//let mut auth_client = AuthClient::connect(address).await.unwrap();

//let mut chat_client = ChatClient::connect(address).await.unwrap();

while (true) {
    let mut request = tonic::Request::new(AuthRequest {
        session_id: Some(session_id.to_string()),
    });
    let response = auth_client.notify_auth2_service(request).await?;
    let result = response.get_ref();
    let result_type = AuthType::from_i32(result.r#type).unwrap();
    if result_type.eq(&AuthType::OAuth2) {
        println!("The server you requested to join has 'OAuth2' Authentication enabled.\n");
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to proceed?")
            .interact()
            .unwrap()
        {
            session_id = response.get_ref().clone().session_id;

        }
    }
}
*/

/*
if Confirm::with_theme(&ColorfulTheme::default())
.with_prompt("Would you like to continue?").interact().unwrap() {
    println!("Executing request...");
    let request = tonic::Request::new(AuthRequest {
        session_id: Some(session_id.to_string())
    });
    if session_id.is_empty() {
        println!("CREATING_SESSION_ID");
        let response = auth_client.notify_auth2_service(request).await?;
        session_id = response.get_ref().session_id.to_string();
    } else {
        println!("EXISTING_SESSION_ID");
        let response = auth_client.notify_auth2_service(request).await?;
        println!("{:?}", response)
    }
} else {
    println!("nevermind then :(");
}
*/

// note: when a user responds yes to start authentication (they send a request that they're ready)
// the server generates an auth_session_id and sends it back to them and that will be used to identify them.
// A URL WILL BE SENT AS WELL AND THE AUTHENTICATION CODE THAT IS DIRECTLY LINKED TO THE CLIENT...
// after 2 minutes the session will be deleted if the user does not authorize it....
