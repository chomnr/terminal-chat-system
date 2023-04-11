use mongodb::options::ClientOptions;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Chatter {
    id: String,
    name: String,
    discriminator: u16,
    session_id: String
}

pub struct ChatterManager {
    redis: redis::Client,
    mongodb: mongodb::Client
}

impl ChatterManager {
    pub fn new(redis: redis::Client, mongodb: mongodb::Client) -> Self {
        Self { redis, mongodb }
    }

    /*
    pub fn verify(&self) {
        &self.mongodb.default_database().unwrap().collection("name")
    }

    fn create(&self) {}
    */
}