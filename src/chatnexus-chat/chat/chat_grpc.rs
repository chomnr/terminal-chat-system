use crate::{chatnexus_chat::{chat_server::Chat, ChatRequest, ChatResponse, ChatFilter, self, Empty}, chat::UserMessage};

use super::ChatService;

use std::{error::Error, io::ErrorKind, net::ToSocketAddrs, pin::Pin, time::Duration};

use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt, Stream};
use tonic::{Request, Response, Status};

type ChatResult<T> = Result<Response<T>, Status>;
type ChatResponseStream = Pin<Box<dyn Stream<Item = Result<ChatResponse, Status>> + Send>>;

// Stores messages with id..


#[tonic::async_trait]
impl Chat for ChatService {
    type RecieveMessageStream = ChatResponseStream;

    async fn send_message(
        &self,
        req: Request<ChatRequest>,
    ) -> ChatResult<Empty> {
        let data = req.get_ref();
        match self.get_chat_session(&data.session_id).await {
          /* Authorized */  Ok(val) => {
                self.insert_into_messages(UserMessage { 
                    username: val.username, 
                    discriminator: val.discriminator, 
                    message: data.message.clone() 
                }).await;
                Ok(Response::new(Empty{}))
            },
          /* Not Authorized */ Err(_) => return Ok(Response::new(Empty{})), 
        }
    }

    async fn recieve_message(
        &self,
        req: Request<ChatFilter>,
    ) -> ChatResult<Self::RecieveMessageStream> {
        todo!()
    }
}