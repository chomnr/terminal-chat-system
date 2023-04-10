use routes::routes;

use crate::oauth2::{OAuth2Config, OAuth2};

mod routes;
mod chatter;
mod oauth2;

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().ok();
    let oauth2_client = OAuth2::new(OAuth2Config::default());
    rocket::build()
    .mount("/", routes())
        .manage(oauth2_client)
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