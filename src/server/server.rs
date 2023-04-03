use std::net::{IpAddr, SocketAddr};

use services::{auth::{self}, chat::chatnexus_chat::chat_server::ChatServer};
use tonic::{transport::Server, Request, Response, Status};
use auth::chatnexus_auth::AuthType;
use crate::services::{auth::AuthService, chat::ChatService};
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Services (Routes)
    let chat = ChatService::new();
    let auth = AuthService::new(AuthType::None);
    // IP Address /w port
    let addr: SocketAddr = "[::1]:50051".parse().unwrap();
    // Indicator that it is running.
    println!("Running ChatService listening on {}", addr);
    // Building the server and serving it.
    Server::builder()
        .add_service(chat.service())
        .add_service(auth.service())
        .serve(addr)
        .await.unwrap();

    Ok(())
}