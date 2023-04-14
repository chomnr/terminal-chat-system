use std::{
    io::{self, Write, BufRead},
    thread, time,
};

use chatnexus_chat::{chat_client::ChatClient, AuthStage, AuthStatus};
use dialoguer::{console::Term, theme::ColorfulTheme, Confirm, Input};

use crate::chatnexus_chat::{
    auth_client::AuthClient, AuthRequest, AuthType, ChatFilter, ChatRequest, Empty,
};

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
    // Connecting to ChatService
    let mut chat_client = ChatClient::connect(address).await.unwrap();
    // Client's Session ID
    let mut session_id = String::default();
    let mut trick_repeat = false;
    let mut at_message = 0;
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
            session_id: String::default(),
            message: String::default(),
        };
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

        while chat_request.session_id.is_empty() {
            if !auth_request.session_id.is_empty() {
                let res = auth_client
                    .promote_stage(auth_request.clone())
                    .await
                    .unwrap();
                let data = res.get_ref();

                if data.stage() == AuthStage::Authorization {
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
                    }
                }
                if data.stage() == AuthStage::Completed {
                    if trick_repeat == true {
                        Term::stdout().clear_screen().unwrap();

                        println!(
                            "\n  {}",
                            console::style("Authorization Approved.")
                                .bold()
                                .green()
                                .bright()
                        );
                        thread::sleep(time::Duration::from_secs(2));
                        chat_request.session_id = data.session_id.to_string();
                        trick_repeat = false;
                        Term::stdout().clear_screen().unwrap();
                    }
                }
            }
        }

        if !chat_request.session_id.is_empty() {
            let mut response_stream = chat_client
                .recieve_message(ChatFilter {
                    session_id: chat_request.session_id.to_string(),
                })
                .await?
                .into_inner();

                tokio::spawn(async move {
                    loop {
                        match response_stream.message().await {
                            Ok(v) => {
                                if v.is_some() {
                                    let user = v.unwrap();
                                    println!("{}#{}: {}", user.username, user.discriminator, user.message);
                                }
                            }
                            Err(_) => {}
                        }
                    }
                });
                
                loop {
                    print!(">> "); 
                    std::io::stdout().flush().unwrap();
                    let mut string: String = String::new();
                    std::io::stdin().read_line(&mut string).unwrap();
                    chat_client.send_message(ChatRequest {
                        session_id: chat_request.session_id.to_string(),
                        message: string
                    }).await.unwrap();
                }


        }
    }
    Ok(())
}

/*
   loop {
                    let stream = response_stream.message().await {
                        let sent_user = stream.unwrap();

                match sent_user {
                    Some(v) => {
                        println!("{}#{}: {}", v.username, v.discriminator, v.message);
                    }
                    None => {}
                }
                    }
                }


/*
            while let stream = response_stream.message().await {
                let sent_user = stream.unwrap();

                match sent_user {
                    Some(v) => {
                        println!("{}#{}: {}", v.username, v.discriminator, v.message);
                    }
                    None => {}
                }
            }
            */
let mut auth_request = AuthRequest {
    session_id: String::default(),
};
let mut chat_request = ChatRequest {
    session_id: session_id,
    message: String::default(),
};
*/

/*
    // check if chatter session exists...
    if chat_request.clone().session_id.is_empty() {
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
                thread::sleep(time::Duration::from_secs(1));
                Term::stdout().clear_screen().unwrap();
            }
        }
    } else {
        let mut movable_chat_client = chat_client.clone();
        let mut movable_session_id = chat_request.session_id.to_string();
        let mut movable_at_message = at_message.clone();

        tokio::spawn( async move {
            let response = movable_chat_client.recieve_message(ChatFilter {
                session_id: movable_session_id,
                at_message: movable_at_message
            }).await.unwrap();
            let data = response.get_ref();
            at_message = data.at_message;
            println!("{}#{}: {}", data.username, data.discriminator, data.message)
        });
        let input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("> ")
            .interact_text()
            .unwrap();
        chat_request.message = input.to_string();
        chat_client.send_message(chat_request.clone()).await.unwrap();
    }
}
*/

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
