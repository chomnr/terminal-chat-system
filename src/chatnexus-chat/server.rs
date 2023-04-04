use std::net::{IpAddr, SocketAddr};

use tonic::transport::Channel;
use tonic::{transport::Server, Request, Response, Status};
use crate::auth::AuthService;
use crate::chat::ChatService;
use chat::chatnexus_chat::AuthType;

mod chat;
mod auth;

/// Information about the gRPC.
const SERVER_NAME: &str = "NexusChat";
const ADDRESS: &str = "[::1]:50051";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chat = ChatService::new();
    let mut auth = AuthService::new(AuthType::None);
    auth.reset_service(); // reset the service.
    
    let addr: SocketAddr = ADDRESS.parse().unwrap();
    println!("    _____ _           _   _   _                     ");
    println!(r"  / ____| |         | | | \ | |                    ");
    println!(r" | |    | |__   __ _| |_|  \| | _____  ___   _ ___ ");
    println!(r" | |    | '_ \ / _` | __| . ` |/ _ \ \/ / | | / __|");
    println!(r" | |____| | | | (_| | |_| |\  |  __/>  <| |_| \__ \");
    println!(r"  \_____|_| |_|\__,_|\__|_| \_|\___/_/\_\\__,_|___/");

    Server::builder()
        .add_service(chat.service())
        .add_service(auth.service())
        .serve(addr)
        .await?;

    /*
    let mut client = AuthClient::connect(AUTH_SERVER_ADDR).await.unwrap();
    let test = Channel::from_shared(AUTH_SERVER_ADDR).unwrap().connect().await.unwrap();

    let request = tonic::Request::new(AuthRequest {
        r#type: AuthType::Discord.into(),
        message: "Tonic".into(),
    });
    let response = client.test(request).await?;
    */
    /* 
    tokio::spawn(Server::builder().add_service(chat.service()).serve(addr));

    let request = tonic::Request::new(AuthRequest {
        r#type: AuthType::Discord.into(),
        message: "Tonic".into(),
    });
    let mut client = tokio::spawn(AuthClient::connect(AUTH_SERVER_ADDR)).await.unwrap().unwrap();

    let request = tonic::Request::new(AuthRequest {
        r#type: AuthType::Discord.into(),
        message: "Tonic".into(),
    });
    let response = client.test(request).await?;
    */

   // println!("RESPONSE={:?}", response);
/* 
    // Services (Routes)
    let chat = ChatService::new();
    // Channels
    // IP Address /w port
    let addr: SocketAddr = "[::1]:50051".parse().unwrap();
    // Indicator that it is running.
    println!("Running ChatService listening on {}", addr);
    // Building the server and serving it.
    tokio::spawn(Server::builder()
        .add_service(chat.service())
        .serve(addr)
        .await?);
    let request = tonic::Request::new(AuthRequest {
        r#type: AuthType::Discord.into(),
        message: "Tonic".into(),
    });
    let mut client = AuthClient::connect(AUTH_SERVER_ADDR).await.unwrap();
    let response = client.test(request).await?;

    println!("RESPONSE={:?}", response);
    */
    Ok(())
}