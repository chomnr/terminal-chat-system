
use chatnexus_chat::auth_client::AuthClient;
use rocket::futures::lock::Mutex;
use routes::routes;

use crate::oauth2::{OAuth2Config, OAuth2};

mod routes;
mod oauth2;

pub mod chatnexus_chat {
    tonic::include_proto!("chatnexus.chat");
}

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().ok();
    // OAuth2
    let oauth2_client = OAuth2::new(OAuth2Config::default());
    // rGPC 
    let address = "http://[::1]:50051";
    let auth_client = Mutex::new(AuthClient::connect(address).await.unwrap());
    // Rocket
    rocket::build()
    .mount("/", routes())
        .manage(oauth2_client)
        .manage(auth_client)
        .ignite().await?
        .launch().await?;
    Ok(())
}
