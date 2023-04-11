use std::fmt::format;
use std::net::{IpAddr, SocketAddr};

use dotenv::dotenv;
use redis::AsyncCommands;
use tonic::transport::Channel;
use tonic::{transport::Server, Request, Response, Status};
use crate::auth::AuthService;
use crate::chat::ChatService;
use crate::chatnexus_chat::AuthType;

mod auth;
mod helper;
mod chat;

pub mod chatnexus_chat {
    tonic::include_proto!("chatnexus.chat");
}

/// Information about the gRPC.
const SERVER_NAME: &str = "Test Chat Server Name";
const ADDRESS: &str = "[::1]:50051";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // .env
    dotenv::dotenv().ok();
    // Databases
    let redis_url = format!("redis://:{}@{}:{}", 
        dotenv::var("REDIS_PASSWORD").unwrap(),
        dotenv::var("REDIS_HOST").unwrap(),
        dotenv::var("REDIS_PORT").unwrap());
    let redis = redis::Client::open(redis_url).unwrap();
    
    //let mut redis_conn = redis.get_async_connection().await.unwrap();
    // Services
    let chat = ChatService::new(redis.clone());
    let auth = AuthService::new(AuthType::OAuth2, redis);
        
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