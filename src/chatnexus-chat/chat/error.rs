use std::{fmt, cmp::min, any::Any};


pub type ChatResult<T> = Result<T, ChatError>;

#[derive(Debug)]
pub enum ChatError {
    ChatSessionNotFound(String)
}

static PREFIX: &str = "[SYSTEM]";

impl fmt::Display for ChatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChatError::ChatSessionNotFound(session_id) => write!(f, "{} unable to locate the chat session {}.", PREFIX, session_id),
        }
    }
}