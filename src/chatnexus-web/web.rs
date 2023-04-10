use oauth2::{basic::BasicClient, ClientSecret, ClientId, AuthUrl, TokenUrl, RedirectUrl, Scope, CsrfToken, PkceCodeChallenge};
use rocket::{
    data::{Limits, ToByteUnit},
    get,
    http::Method,
    routes,
    serde::json::{serde_json::json, Value},
};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use routes::routes;

mod routes;

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
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
    // oauth2
    let client =
    BasicClient::new(
        ClientId::new(dotenv::var("OAUTH2_CLIENT_ID").unwrap()),
        Some(ClientSecret::new(dotenv::var("OAUTH2_CLIENT_SECRET").unwrap())),
        AuthUrl::new(dotenv::var("OAUTH2_AUTHORIZE").unwrap()).unwrap(),
        Some(TokenUrl::new(dotenv::var("OAUTH2_TOKEN").unwrap()).unwrap())
    )
    .set_redirect_uri(RedirectUrl::new(dotenv::var("OAUTH2_REDIRECT_URI").unwrap()).unwrap());

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

// Generate the full authorization URL

    rocket::build()
    .mount("/", routes())
        .manage(client)
        .attach(cors)
        .ignite()
        .await?
        .launch()
        .await?;
    Ok(())
}
