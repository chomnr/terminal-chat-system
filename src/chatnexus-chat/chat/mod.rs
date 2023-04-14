use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

use tokio_stream::Stream;
use tokio::sync::{mpsc, RwLock, futures};
use tonic::transport::Server;
use tonic::{Request, Response, Status, Streaming};

use redis::AsyncCommands;
use serde::{Serialize, Deserialize};

use crate::chatnexus_chat::ChatResponse;
use crate::chatnexus_chat::{chat_server::ChatServer, ChatRequest};

use self::error::{ChatError, ChatResult};

mod chat_grpc;
mod error;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatUser {
    pub uid: String,
    pub username: String,
    pub discriminator: String,
    pub session_id: String
}

#[derive(Clone, Debug)]
pub struct UserMessage {
    pub username: String,
    pub discriminator: String,
    pub message: String
}

#[derive(Clone, Debug)]
pub struct ChatService {
    pub service: Option<ChatServer<Self>>,
    pub redis: redis::Client,
    pub senders: Arc<RwLock<HashMap<String, mpsc::Sender<ChatResponse>>>>,
}

impl ChatService {
    pub fn new(redis_cli: redis::Client) -> Self {
        let mut chat_service = Self {
            service: None,
            redis: redis_cli,
            senders: Arc::new(RwLock::new(HashMap::new()))
        };
        chat_service.service = Some(ChatServer::new(chat_service.clone()));
        chat_service
    }

    /// Looks for the user's chat session id inside redis if
    /// found returns [ChatUser], if not returns [ChatError].
    ///
    /// # Arguments
    ///
    /// * `session_id` - Session id of client.
    ///
    /// ```
    pub async fn get_chat_session(&self, session_id: &str) -> ChatResult<ChatUser> {
        let conn = &mut self.redis.get_async_connection().await.unwrap();
        let key = format!("chat-session:{}", session_id).to_string();
        let session: String = conn
            .get(key)
            .await
            .map_err(|_| ChatError::ChatSessionNotFound(session_id.to_string()))?;
        Ok(serde_json::from_str(&session).unwrap())
    }

    pub async fn broadcast(&self, msg: ChatResponse) {
        let senders = self.senders.read().await;
        for (name, tx) in senders.iter() {
            match tx.send(msg.clone()).await {
                Ok(_) => {}
                Err(_) => {
                    println!("[Broadcast] SendError: to {}, {:?}", name, msg)
                }
            }
        }
    }
    
    pub fn service(self) -> ChatServer<ChatService> {
        self.service.unwrap()
    }
}