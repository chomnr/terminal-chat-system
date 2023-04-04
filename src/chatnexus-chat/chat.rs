use std::net::{IpAddr, SocketAddr};

use chatnexus_chat::{chat_server::ChatServer, ChatResponse, ChatRequest, Empty};
use tonic::{transport::Server, Request, Response, Status};

use self::chatnexus_chat::{chat_server::{Chat, self}, BarenResponse};


pub mod chatnexus_chat {
    tonic::include_proto!("chatnexus.chat");
}

#[derive(Default, Clone)]
pub struct ChatNexusChat {}

#[tonic::async_trait]
impl Chat for ChatNexusChat {
    async fn send_message(&self, request: Request<ChatRequest>) -> Result<Response<ChatResponse>, Status>  {
        println!("{}: {}", &request.get_ref().username, &request.get_ref().message);
        let chat_response = ChatResponse {
            status: true
        };
        Ok(Response::new(chat_response))
    }
    
    async fn recieve_message(&self, request: Request<Empty>) -> Result<Response<ChatRequest>, Status>  {
        println!("test");
        todo!()
    }
}

pub struct ChatService {
    chat: ChatNexusChat,
    service: ChatServer<ChatNexusChat>,
}

impl ChatService {
    pub fn new() -> Self {
        let chat = ChatNexusChat::default();
        let service = chat_server::ChatServer::new(chat.clone());
        Self {
            chat,
            service,
        }
    }
    pub fn service(self) -> ChatServer<ChatNexusChat> {
        self.service
    }
}