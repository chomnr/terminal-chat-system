use std::{
    io::{self, stdin, stdout, Write},
    sync::Arc,
    thread, time,
};

use chatnexus_chat::{chat_client::ChatClient, AuthStage, AuthStatus};
use dialoguer::{
    console::Term,
    theme::{self, ColorfulTheme, SimpleTheme},
    Confirm, Input,
};
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
    // Enable colors...
    console::set_colors_enabled(true);
    // Address of the server we would like to connect to.
    let address = "http://[::1]:50051";
    // Connecting to AuthService
    let mut auth_client = AuthClient::connect(address).await.unwrap();
    let mut chat_client = ChatClient::connect(address).await.unwrap();
    let mut trick_repeat = false;
    // Client's Session ID
    let mut session_id = String::default();
    //let mut current_stage = AuthStage::Stage1;
    //let mut waiting = false;
    // Notifying server of Client's presence.
    let notify_presence = auth_client.notify_presence(Empty::default()).await?;
    let presence_result = notify_presence.get_ref();
    // Handle OAuth2 Authorization
    if presence_result.auth_type() == AuthType::OAuth2 {
        let mut auth_request = AuthRequest {
            session_id: String::default(),
        };
        let mut chat_request = ChatRequest {
            session_id: session_id,
            message: String::default(),
        };

        loop {
            // check if chatter session exists...
            if chat_request.session_id.is_empty() {
                if auth_request.clone().session_id.is_empty() {
                    println!(
                        "Server Authorization Method: {:?}\n",
                        AuthType::from_i32(presence_result.auth_type).unwrap()
                    );
                    if Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt("Begin Authorization?")
                        .interact()
                        .unwrap()
                    {
                        let res = auth_client
                            .promote_stage(auth_request.clone())
                            .await
                            .unwrap();
                        auth_request.session_id = res.get_ref().session_id.to_string();
                    }
                } else {
                    let res = auth_client
                        .promote_stage(auth_request.clone())
                        .await
                        .unwrap();
                    if res.get_ref().stage() == AuthStage::Authorization {
                        if trick_repeat == false {
                            Term::stdout().clear_screen().unwrap();
                            println!(
                                "\n  {}",
                                console::style("Waiting for Authentication...")
                                    .bold()
                                    .yellow()
                                    .bright()
                            );
                            println!(
                                "\n  URL: {}",
                                console::style(res.get_ref().url()).bold().yellow().bright()
                            );
                            println!(
                                "\n  Session ID: {}",
                                console::style(res.get_ref().session_id.to_string())
                                    .bold()
                                    .yellow()
                                    .bright()
                            );
                            println!(
                                "\n  Code: {}",
                                console::style(res.get_ref().code())
                                    .bold()
                                    .yellow()
                                    .bright()
                            );
                            trick_repeat = true;
                            //thread::sleep(time::Duration::from_secs(10))
                        }
                    }
                    if res.get_ref().stage() == AuthStage::Completed {
                        if trick_repeat == true {
                            Term::stdout().clear_screen().unwrap();

                            println!(
                                "\n  {}",
                                console::style("Authorization Approved.")
                                    .bold()
                                    .green()
                                    .bright()
                            );
                            trick_repeat = false
                        }
                        chat_request.session_id = res.get_ref().session_id.to_string();
                        thread::sleep(time::Duration::from_secs(3));
                        Term::stdout().clear_screen().unwrap();
                    }
                }
            } else {
                let input: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("> ")
                    .interact_text()
                    .unwrap();
                println!("{}: {}", chat_request.session_id, input);
            }
        }
    }
    Ok(())
}

// check if is authenticated if it is create a separate loop.. that handles just the chatclient requests
/*
loop {
    // [Request/Response] Presence gRPC


    // [Request] Stage Promotion gRPC
    let mut request = AuthRequest {
        session_id: session_id.clone(),
    };

    if request.session_id.is_empty() {
        println!(
            "Server Authorization Method: {:?}\n",
            AuthType::from_i32(presence_result.auth_type).unwrap()
        );
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Begin Authorization?")
            .interact()
            .unwrap()
        {
            // [Response] Stage Promotion gRPC
            let a = auth_client.promote_stage(request.clone()).await.unwrap();
            let b = a.get_ref();
            session_id = b.session_id.to_string();
            current_stage = AuthStage::from_i32(b.stage.unwrap()).unwrap();
        }
    } else {
        if current_stage == AuthStage::Stage1 {
            let a = auth_client.promote_stage(request.clone()).await.unwrap();
            let b = a.get_ref();
            current_stage = AuthStage::from_i32(b.stage.unwrap()).unwrap();
        }

        if current_stage == AuthStage::Stage2 {
            let a = auth_client.promote_stage(request.clone()).await.unwrap();
            let b = a.get_ref();
            current_stage = AuthStage::from_i32(b.stage.unwrap()).unwrap();
        }

        if current_stage == AuthStage::Stage3 {
            let a = auth_client.promote_stage(request.clone()).await.unwrap();
            let b = a.get_ref();
            //current_stage = AuthStage::from_i32(b.stage.unwrap()).unwrap();
            current_stage = AuthStage::from_i32(b.stage.unwrap()).unwrap();
            if waiting == false {
                Term::stdout().clear_screen().unwrap();
                println!("\n  {}", console::style("Waiting for Authentication...").bold().yellow().bright());
                println!("\n  URL: {}", console::style(b.url()).bold().yellow().bright());
                println!("\n  Session ID: {}", console::style(b.session_id.to_string()).bold().yellow().bright());
                println!("\n  Code: {}", console::style(b.code()).bold().yellow().bright());
                waiting = true
            }
        }
        if current_stage == AuthStage::Completed {
            if waiting == false {
                Term::stdout().clear_screen().unwrap();
                println!("\n  {}", console::style("Authorization Approved.").bold().green().bright());
                waiting = true
            }
        }
        */
