use std::{collections::HashMap, sync::Arc};

use redis::AsyncCommands;
use rocket::futures::lock::Mutex;
use serde::{Serialize, Deserialize};

use crate::chatnexus_chat::chat_server::ChatServer;

use self::error::{ChatError, ChatResult};

mod chat_grpc;
mod error;

#[derive(Clone, Serialize, Deserialize)]
pub struct ChatUser {
    pub uid: String,
    pub username: String,
    pub discriminator: String,
    pub session_id: String
}

#[derive(Clone)]
pub struct UserMessage {
    pub username: String,
    pub discriminator: String,
    pub message: String
}

#[derive(Clone)]
pub struct ChatService {
    pub service: Option<ChatServer<Self>>,
    pub redis: redis::Client,
    pub messages: Arc<Mutex<HashMap<usize, UserMessage>>>
}

impl ChatService {
    pub fn new(redis_cli: redis::Client) -> Self {
        let mut chat_service = Self {
            service: None,
            redis: redis_cli,
            messages: Arc::new(Mutex::new(HashMap::new()))
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

    // test
    pub async fn insert_into_messages(&self, message: UserMessage) {
        let id = self.messages.lock().await.len();
        self.messages.lock().await.insert(id, message).unwrap();
    }
    
    pub fn service(self) -> ChatServer<ChatService> {
        self.service.unwrap()
    }
}