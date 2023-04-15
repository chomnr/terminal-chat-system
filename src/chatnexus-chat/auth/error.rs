use std::{fmt};


pub type AuthResult<T> = Result<T, AuthError>;

#[derive(Debug)]
pub enum AuthError {
    SessionNotFound(String),
    SessionValidationFailed(String),
    FailedToUpdateSession(String),
    FailedToCreateSession(String)
}

static PREFIX: &str = "[SYSTEM]";

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::SessionNotFound(session_id) => write!(f, "{} unable to locate the session {}.", PREFIX, session_id),
            AuthError::SessionValidationFailed(session_id) => write!(f, "{} failed to validate session {}.", PREFIX, session_id),
            AuthError::FailedToUpdateSession(session_id) => write!(f, "{} failed to update {}.", PREFIX, session_id),
            AuthError::FailedToCreateSession(session_id) => write!(f, "{} failed to create session for {}.", PREFIX, session_id),
        }
    }
}