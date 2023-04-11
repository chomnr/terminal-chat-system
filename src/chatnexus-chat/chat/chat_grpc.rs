use crate::chatnexus_chat::{chat_server::Chat, ChatRequest, ChatResponse, Empty};

use super::ChatService;

use std::net::{IpAddr, SocketAddr};

use tonic::{transport::Server, Request, Response, Status};


#[tonic::async_trait]
impl Chat for ChatService {
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
