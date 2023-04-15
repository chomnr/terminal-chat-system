use std::{
    collections::HashMap,
    fs::read,
    io::{self, stdout, LineWriter, Write},
    process::Command,
    sync::Arc,
    thread,
    time::{self, Duration},
};

use rand::Rng;

use crate::chatnexus_chat::{
    auth_client::AuthClient, AuthRequest, AuthType, ChatFilter, ChatRequest, Empty,
};
use chatnexus_chat::{chat_client::ChatClient, AuthStage, AuthStatus, ChatResponse};
use dialoguer::{
    console::{style, Term},
    theme::{ColorfulTheme, SimpleTheme},
    Confirm, Editor, Input,
};
use rand::thread_rng;
use std::error::Error;
use tokio::sync::mpsc;

use std::sync::Mutex;
use unicode_width::UnicodeWidthStr;

pub mod chatnexus_chat {
    tonic::include_proto!("chatnexus.chat");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Address of the server we would like to connect to.
    let address = "http://[::1]:50051";
    // Connecting to AuthService.
    let mut auth_client = AuthClient::connect(address).await.unwrap();
    // Connecting to ChatService.
    let mut chat_client = ChatClient::connect(address).await.unwrap();
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
                        println!("\n    Waiting for Authentication.");
                        println!("\n    Url: {}", data.url());
                        println!("\n    Session: {}", data.session_id);
                        println!("\n    Code: {}", data.code());
                        print_text = true;
                    }
                }
                if data.stage().eq(&AuthStage::Completed) {
                    if print_text == true {
                        Term::stdout().clear_screen().unwrap();
                        println!("\n    Authentication Successful.");
                        print_text = false;
                        chat_request.session_id = data.session_id.to_string();
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

/*


                let chat_storage_clone = chat_storage.clone();
                tokio::spawn(async move {
                    loop {
                        match r_stream.message().await {
                            Ok(v) => {
                                if v.is_some() {
                                let user = v.clone().unwrap();
                                chat_storage_clone.lock().unwrap().push(user.clone());
                                let len = chat_storage.lock().unwrap().len() as u16;
                                let storage = chat_storage.lock().unwrap();
                                for i in 0..len as usize {
                                    execute!(stdout(), cursor::MoveTo(0, len)).unwrap();
                                    execute!(
                                        stdout(),
                                        SetForegroundColor(Color::Red),
                                        Print(format!("{}#{}: {}", storage[i].username, storage[i].discriminator, storage[i].message).to_string())
                                    )
                                    .unwrap();
                                }
                            }
                            }
                            Err(_) => {}
                        }
                    }
                });
    tokio::spawn(async move {
                    loop {
                        if let Ok(Some(next_message)) = r_stream.message().await {
                            window.printw(&format!("{:?}", next_message).to_string());
                        }
                    }
                });
                loop {
                    let msg: String = Input::new().with_prompt(">> ").interact_text().unwrap();
                    chat_client
                        .send_message(ChatRequest {
                            session_id: chat_request.session_id.to_string(),
                            message: msg,
                        })
                        .await
                        .unwrap();
                }

tokio::spawn(async move {
                        loop {
                            if let Ok(Some(next_message)) = stream.message().await {
                                Term::stdout().clear_screen().unwrap();
                                window.printw(&format!("{:?}", next_message).to_string());
                                // println!("{:?}", next_message);
                            }
                        }
                    }
                tokio::spawn(async move {
                    loop {
                        let msg: String = Input::new().with_prompt(">> ").interact_text().unwrap();
                        chat_client
                            .send_message(ChatRequest {
                                session_id: chat_request.session_id.to_string(),
                                message: msg,
                            })
                            .await
                            .unwrap();
                    }
                });*/

/*
let task_2 = async {
    loop {
        let msg: String = Input::new().with_prompt(">> ").interact_text().unwrap();
        chat_client
            .send_message(ChatRequest {
                session_id: chat_request.session_id.to_string(),
                message: msg,
            })
            .await
            .unwrap();
    }
};*/

/*

loop {
        match stream.message().await {
            Ok(_) => {},
            Err(_) => {}
        }
    }
tokio::spawn(async move {
    loop {
        match stream.message().await {
            Ok(_) => {},
            Err(_) => {}
        }
    }
});*/

/*
loop {
    let msg: String = Input::new().with_prompt(">> ").interact_text().unwrap();
    chat_client
    .send_message(ChatRequest {
        session_id: chat_request.session_id.to_string(),
         message: msg,
    })
    .await
    .unwrap();
}*/
/*

  let chat_storage_clone = chat_storage.clone();
                tokio::spawn(async move {
                    loop {
                        match response_stream.message().await {
                            Ok(v) => {
                                if v.is_some() {
                                    Term::stdout().clear_screen().unwrap();
                                    let user = v.unwrap();
                                    let mut storage = chat_storage_clone.lock().unwrap();
                                    storage.push(user.clone());
                                    let mut result = String::default();
                                    for i in 0..storage.len() {
                                        result += &format!(
                                            "\n     {}{}: {}",
                                            style(storage[i].username.to_string()).blue().bright(),
                                            style(format!("#{}", storage[i].discriminator))
                                                .white()
                                                .bright(),
                                            style(storage[i].message.to_string()).white().bright()
                                        ).to_string();
                                    }
                                    println!("{}\n\n", result);
                                }
                            }
                            Err(_) => {}
                        }
                    }
                });

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App, mut input_buffer: String) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app, input_buffer.clone()))?;
        if let Event::Key(KeyEvent { code: KeyCode::Char(c), modifiers: KeyModifiers::NONE, kind, state }) = crossterm::event::read().unwrap() {
            if c == '\n' {
                println!("Input received: {}", input_buffer);
                input_buffer.clear();
            } else {
                input_buffer.push(c);
            }
        }
    }
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App, input_buffer: String) {
    let chunks = Layout::default()
                .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
                .split(f.size());

            let text = vec![
                Spans::from(Span::styled(">> ", Style::default().fg(Color::White))),
                Spans::from(Span::styled(input_buffer.as_str(), Style::default().fg(Color::Yellow))),
            ];
            let input = Paragraph::new(text)
                .style(Style::default().fg(Color::White))
                .block(tui::widgets::Block::default().borders(tui::widgets::Borders::ALL).title("Input"));
            f.render_widget(input, chunks[0]);

            let message = Paragraph::new("Some message");
            f.render_widget(message, chunks[1]);

}
*/
/*
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
                    style("Waiting for Authentication...")
                        .bold()
                        .yellow()
                        .bright()
                );
                println!(
                    "\n  URL: {}",
                    style(res.get_ref().url()).bold().yellow().bright()
                );
                println!(
                    "\n  Session ID: {}",
                    style(res.get_ref().session_id.to_string())
                        .bold()
                        .yellow()
                        .bright()
                );
                println!(
                    "\n  Code: {}",
                    style(res.get_ref().code()).bold().yellow().bright()
                );
                trick_repeat = true;
            }
        }
        if data.stage() == AuthStage::Completed {
            if trick_repeat == true {
                Term::stdout().clear_screen().unwrap();

                println!(
                    "\n  {}",
                    style("Authorization Approved.").bold().green().bright()
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



}
*/

// let chat_storage_clone = chat_storage.clone();
/*

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
                    let mut storage = chat_storage_clone.lock().await;
                    storage.push(user.clone());
                    for message in chat_storage.lock().await.iter() {
                        println!(
                            "{}{}: {}",
                            console::style(message.username.to_string()).blue().bright(),
                            console::style(format!("#{}", message.discriminator))
                                .white()
                                .bright(),
                            console::style(message.message.to_string()).white().bright()
                        );
                    }
                }
            }
            Err(_) => {}
        }
    }
});
*/
/*
loop {
    let msg: String = Input::new().with_prompt(">> ").interact_text().unwrap();
    chat_client
    .send_message(ChatRequest {
        session_id: chat_request.session_id.to_string(),
         message: msg,
    })
    .await
    .unwrap();
}
*/
