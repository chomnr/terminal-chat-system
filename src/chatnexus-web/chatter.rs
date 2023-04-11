use mongodb::options::ClientOptions;
use serde::{Serialize, Deserialize};
use redis::{AsyncCommands, RedisResult};

use crate::chatnexus_chat::AuthStage;

#[derive(Serialize, Deserialize)]
pub struct Chatter {
    id: String,
    name: String,
    discriminator: u16,
    session_id: String
}

// Authentication Struct.
#[derive(Clone, Serialize, Deserialize)]
pub struct AuthSession {
    session_id: String,
    stage: AuthStage,
    url: Option<String>,
    code: Option<String>,
}

pub struct ChatterManager {
    redis: redis::Client,
    mongodb: mongodb::Client
}

impl ChatterManager {
    pub fn new(redis: redis::Client, mongodb: mongodb::Client) -> Self {
        Self { redis, mongodb }
    }

    pub async fn verify(&self, session_id: &str, code: &str) {
        let mut conn = self.redis.get_async_connection().await.unwrap();
        // if successful convert Stage to Completed
        // store info about the user inside mongodb the database.
        // create chatter:<session_id> with the Chatter struct.
        // delete auth-session:<session_id>
        // yeah that's that.
        //let test: String = conn.get(session_id).await.unwrap();
    }
}