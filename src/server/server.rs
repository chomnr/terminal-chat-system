use std::net::{IpAddr, SocketAddr};

use chatnexus_chat::{chat_service_server::ChatService, ChatResponse, ChatRequest, Empty};
use chatnexus_chat::chat_service_server::ChatServiceServer;
use tonic::{transport::Server, Request, Response, Status};

use crate::chatnexus_chat::chat_service_server;

pub mod chatnexus_chat {
    tonic::include_proto!("chatnexus.chat");
}

#[derive(Default, Clone)]
pub struct ChatNexusChat {}

#[tonic::async_trait]
impl ChatService for ChatNexusChat {
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

#[tokio::main]
async fn main(){
    let addr: SocketAddr = "[::1]:50051".parse().unwrap();
    let chat_service = ChatNexusChat::default();
    println!("Running ChatService listening on {}", addr);
    Server::builder()
        .add_service(ChatServiceServer::new(chat_service))
        .serve(addr)
        .await.unwrap();
}

/*= v
pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
*/