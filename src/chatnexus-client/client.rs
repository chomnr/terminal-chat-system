use std::{
    sync::{Arc, Mutex},
    time::{Duration},
};

use crate::chatnexus_chat::{
    auth_client::AuthClient, AuthRequest, AuthType, ChatFilter, ChatRequest, Empty,
};
use chatnexus_chat::{chat_client::ChatClient, AuthStage, ChatResponse};
use dialoguer::{
    console::{style, Term},
    theme::{ColorfulTheme},
    Confirm, Input,
};

pub mod chatnexus_chat {
    tonic::include_proto!("chatnexus.chat");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Address of the server we would like to connect to. {"http://[::1]:50051"}
    let msg: String = Input::new().with_prompt("Connect to").interact_text().unwrap();
    let address = msg;
    Term::stdout().clear_screen().unwrap();
    // Connecting to AuthService.
    let mut auth_client = AuthClient::connect(address.to_string()).await.unwrap();
    // Connecting to ChatService.
    let mut chat_client = ChatClient::connect(address.to_string()).await.unwrap();
    // Prevents repeating text.
    let mut print_text = false;
    // Chat Storage
    let chat_storage: Arc<Mutex<Vec<ChatResponse>>> = Arc::new(Mutex::new(Vec::new()));
    // gRPC NotifyPresence request.
    let notify_presence = auth_client.notify_presence(Empty::default()).await?;
    let presence_result = notify_presence.get_ref();
    // Empty Requests.
    let mut auth_request = AuthRequest {
        session_id: String::default(),
    };
    let mut chat_request = ChatRequest {
        session_id: String::default(),
        message: String::default(),
    };
    // Chat Window
    if presence_result.auth_type() == AuthType::OAuth2 {
        println!(
            "Authorization Method: {}\n",
            presence_result.auth_type().as_str_name()
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
        loop {
            std::thread::sleep(Duration::from_secs(2));
            if !auth_request.session_id.is_empty() && chat_request.session_id.is_empty() {
                let res = auth_client
                    .promote_stage(auth_request.clone())
                    .await
                    .unwrap();
                let data = res.get_ref();

                if data.stage().eq(&AuthStage::Authorization) {
                    if print_text == false {
                        Term::stdout().clear_screen().unwrap();
                        println!("{}", style("\n    Waiting for Authentication.").yellow().bright());
                        println!("\n    Url: {}", data.url());
                        println!("\n    Session: {}", data.session_id);
                        println!("\n    Code: {}", data.code());
                        print_text = true;
                    }
                }
                if data.stage().eq(&AuthStage::Completed) {
                    if print_text == true {
                        Term::stdout().clear_screen().unwrap();
                        println!("{}", style("\n    Authentication Successful.").green().bright());
                        print_text = false;
                        chat_request.session_id = data.session_id.to_string();
                        std::thread::sleep(Duration::from_secs(1));
                    }
                }
            }
            if !chat_request.session_id.is_empty() {
                if print_text == false {
                    Term::stdout().clear_screen().unwrap();
                    println!("You're currently connected to {}", address);
                    print_text = true;
                }
                let mut r_stream = chat_client
                    .recieve_message(ChatFilter {
                        session_id: chat_request.session_id.to_string(),
                    })
                    .await?
                    .into_inner();

                let chat_storage_clone = chat_storage.clone();
                tokio::spawn(async move {
                    loop {
                        match r_stream.message().await {
                            Ok(v) => {
                                if v.is_some() {
                                    let user = v.clone().unwrap();
                                    chat_storage_clone.lock().unwrap().push(user);
                                    let len = chat_storage_clone.lock().unwrap().len();
                                    let search = chat_storage_clone.lock().unwrap();
                                    Term::stdout().clear_screen().unwrap();
                                    for i in 0..len {
                                        println!(
                                            "{}#{}: {}",
                                            style(search[i].username.to_string()).blue().bright(),
                                            style(search[i].discriminator.to_string())
                                                .red()
                                                .bright(),
                                            style(search[i].message.to_string()).white()
                                        )
                                    }
                                    print!(">> \x1B[{};0H", len + 3); 
                                }
                            }
                            Err(_) => {}
                        }
                    }
                });

                loop {
                    print!("\x1B[{};0H", chat_storage.lock().unwrap().len() + 3); 
                    let msg: String = Input::new().with_prompt(">> ").interact_text().unwrap();
                    chat_client
                        .send_message(ChatRequest {
                            session_id: chat_request.session_id.to_string(),
                            message: msg,
                        })
                        .await
                        .unwrap();
                    print!("\x1B[{};0H\x1B[K", chat_storage.lock().unwrap().len() + 4);
                }
            }
        }
    }
    Ok(())
}
