use std::{fmt, cmp::min, any::Any};


pub type AuthResult<T> = Result<T, AuthError>;

#[derive(Debug)]
pub enum AuthError {
    SessionNotFound(String),
    FailedToUpdateSession(String)
}

static PREFIX: &str = "[SYSTEM]";

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::SessionNotFound(session_id) => write!(f, "{} unable to locate the session {}.", PREFIX, session_id),
            AuthError::FailedToUpdateSession(session_id) => write!(f, "{} failed to update {}.", PREFIX, session_id),
        }
    }
}