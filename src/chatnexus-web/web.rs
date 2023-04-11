use std::fmt::format;

use chatter::ChatterManager;
use mongodb::options::ClientOptions;
use routes::routes;

use crate::oauth2::{OAuth2Config, OAuth2};

mod routes;
mod chatter;
mod oauth2;

pub mod chatnexus_chat {
    tonic::include_proto!("chatnexus.chat");
}

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().ok();
    // Databases
        // Redis
    let redis_url = format!("redis://:{}@{}:{}", 
        dotenv::var("REDIS_PASSWORD").unwrap(),
        dotenv::var("REDIS_HOST").unwrap(),
        dotenv::var("REDIS_PORT").unwrap());
    let redis = redis::Client::open(redis_url).unwrap();
        // Mongodb
    let mongodb_url = format!("mongodb+srv://{}:{}@{}/?retryWrites=true&w=majority",
        dotenv::var("MONGODB_USERNAME").unwrap(),
        dotenv::var("MONGODB_PASSWORD").unwrap(),
        dotenv::var("MONGODB_HOST").unwrap());
    let mut mongodb_options = ClientOptions::parse(mongodb_url).await.unwrap();
    let mongodb = mongodb::Client::with_options(mongodb_options).unwrap();  
    // OAuth2
    let oauth2_client = OAuth2::new(OAuth2Config::default());
    // Chatter
    let chatter_manager = ChatterManager::new(redis, mongodb);
    rocket::build()
    .mount("/", routes())
        .manage(oauth2_client)
        .manage(chatter_manager)
        .ignite().await?
        .launch().await?;
    Ok(())
}


/*
    // dotenv
    dotenv::dotenv().ok();
    // rocket_cors
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::All,
        allowed_methods: vec![Method::Get, Method::Post, Method::Options]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }.to_cors()
    .unwrap();

    let chatter_manager = ChatterManager::new(todo!(), todo!());

    rocket::build()
    .mount("/", routes())
        //.manage(client)
        .attach(cors)
        .manage(state)
        .ignite()
        .await?
        .launch()
        .await?;
*/