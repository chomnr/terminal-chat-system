use std::net::{IpAddr, SocketAddr};

use tonic::transport::Channel;
use tonic::{transport::Server, Request, Response, Status};
use crate::auth::AuthService;
use crate::chat::ChatService;
use chat::chatnexus_chat::AuthType;

mod chat;
mod auth;
mod helper;

/// Information about the gRPC.
const SERVER_NAME: &str = "Test Chat Server Name";
const ADDRESS: &str = "[::1]:50051";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chat = ChatService::new();
    let mut auth = AuthService::new(AuthType::OAuth2);
        
    let addr: SocketAddr = ADDRESS.parse().unwrap();
    println!("    _____ _           _   _   _                     ");
    println!(r"  / ____| |         | | | \ | |                    ");
    println!(r" | |    | |__   __ _| |_|  \| | _____  ___   _ ___ ");
    println!(r" | |    | '_ \ / _` | __| . ` |/ _ \ \/ / | | / __|");
    println!(r" | |____| | | | (_| | |_| |\  |  __/>  <| |_| \__ \");
    println!(r"  \_____|_| |_|\__,_|\__|_| \_|\___/_/\_\\__,_|___/");
    println!("");
    Server::builder()
        .add_service(chat.service())
        .add_service(auth.service())
        .serve(addr)
        .await?;
    Ok(())
}