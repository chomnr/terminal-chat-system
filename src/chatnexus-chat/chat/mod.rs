use crate::chatnexus_chat::chat_server::ChatServer;

mod chat_grpc;

pub struct ChatUser {
    uid: String,
    username: String,
    discriminator: String
}

#[derive(Clone)]
pub struct ChatService {
    pub service: Option<ChatServer<Self>>,
    pub redis: redis::Client,
}

impl ChatService {
    pub fn new(redis_cli: redis::Client) -> Self {
        let mut chat_service = Self {
            service: None,
            redis: redis_cli,
        };
        chat_service.service = Some(ChatServer::new(chat_service.clone()));
        chat_service
    }
    pub fn service(self) -> ChatServer<ChatService> {
        self.service.unwrap()
    }
}