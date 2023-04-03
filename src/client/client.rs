use std::io::{stdin, stdout, Write};

use chatnexus_chat::{
    chat_service_client::{self, ChatServiceClient},
    ChatRequest,
};

pub mod chatnexus_chat {
    tonic::include_proto!("chatnexus.chat");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ChatServiceClient::connect("http://[::1]:50051")
        .await
        .unwrap();
    let mut isLoggedIn = false;
    let mut saved_logged = String::default();

    while true {
        let username = if isLoggedIn {
            saved_logged.clone()
        } else {
            print!("Login as: ");
            stdout().flush().unwrap();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            isLoggedIn = true;
            saved_logged = input.trim().to_string();
            saved_logged.clone()
        };

        print!("Enter your Message: ");
        stdout().flush().unwrap();

        let mut message = String::new();
        stdin().read_line(&mut message).unwrap();

        let request = tonic::Request::new(ChatRequest { username, message });

        let response = client.send_message(request).await.unwrap();

        println!("RESPONSE={:?}", response);
    }
    Ok(())
}
/*
print!("Enter your name: ");
stdout().flush().unwrap(); // flush the output to make sure the prompt is displayed

let mut input = String::new();
stdin().read_line(&mut input).unwrap();

print!("Hello, {}! You typed: {}", input.trim(), input);
*/
//let mut line = String::new();
//let b1 = std::io::stdout().write("Tutorials ".as_bytes()).unwrap();
/*
let mut client = GreeterClient::connect("http://[::1]:50051").await?;

let request = tonic::Request::new(HelloRequest {
    name: "Tonic".into(),
});

let response = client.say_hello(request).await?;

println!("RESPONSE={:?}", response);
*/
