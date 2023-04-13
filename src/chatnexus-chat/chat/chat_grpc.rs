use crate::chatnexus_chat::{chat_server::Chat, ChatRequest, ChatResponse, Empty};

use super::ChatService;

use std::{net::{IpAddr, SocketAddr}, pin::Pin};

use tokio_stream::Stream;
use tonic::{transport::Server, Request, Response, Status};

type ChatResult<T> = Result<Response<T>, Status>;
type ChatResponseStream = Pin<Box<dyn Stream<Item = Result<ChatResponse, Status>> + Send>>;

#[tonic::async_trait]
impl Chat for ChatService {
    type SendMessageStream = ChatResponseStream;
    type RecieveMessageStream = ChatResponseStream;
    async fn send_message(
        &self,
        req: Request<ChatRequest>,
    ) -> ChatResult<Self::SendMessageStream> {
        
        todo!()
    }

    async fn recieve_message(
        &self,
        req: Request<Empty>,
    ) -> ChatResult<Self::RecieveMessageStream> {
        todo!()
    }
}


    /*
    async fn send_message(&self, request: Request<ChatRequest>) -> Result<Response<ChatResponse>, Status>  {
        println!("{}: {}", &request.get_ref().session_id, &request.get_ref().message);
        let chat_response = ChatResponse {
            status: true
        };
        Ok(Response::new(chat_response))
    }
    
    async fn recieve_message(&self, request: Request<Empty>) -> Result<Response<ChatRequest>, Status>  {
        println!("test");
        todo!()
    }
    */