use std::io;

use chatnexus_chat::chat_client::ChatClient;

use crate::chatnexus_chat::{ChatRequest, auth_client::AuthClient, Empty};


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

   // let response = chat_client.send_message(request).await?;
   let request = tonic::Request::new(Empty::default());
   
   let response = auth_client.send_auth_message(request).await?;
   println!("{:?}", response.get_ref().message);

   //let response = auth_client.send_auth_message(request)

    //println!("{:?}", response);

    println!("Going to wait...");
    io::stdin().read_line(&mut String::new()).unwrap();
    Ok(())
}