use oauth2::{Client, ErrorResponse, TokenResponse, TokenType, TokenIntrospectionResponse, RevocableToken, basic::BasicClient, Scope};
use rocket::{Route, routes, get, response::Redirect, State, post, http::CookieJar};
use serde_json::{json, Value};
use urlencoding::encode;

#[get("/?<code>")]
fn index(jar: &CookieJar, code: String, oauth2: &State<BasicClient>) -> Value {
    json!({
        "message": code
    })
}

#[get("/login")]
fn login(oauth2: &State<BasicClient>) -> Redirect {
    let auth_url = format!(
        "{}?response_type=code&client_id={}&scope={}&redirect_uri={}&prompt=consent",
        dotenv::var("OAUTH2_AUTHORIZE").unwrap(),
        dotenv::var("OAUTH2_CLIENT_ID").unwrap(),
        encode(&dotenv::var("OAUTH2_SCOPES").unwrap()),
        encode(&dotenv::var("OAUTH2_REDIRECT_URI").unwrap())
    )
    .to_string();
    Redirect::to(auth_url)
}
/*
#[get("/verify")]
fn verify(oauth2: &State<BasicClient>) -> Value {
    json!({
        "message": "dsasad"
    })
}
 */

#[get("/verify")]
fn verify(oauth2: &State<BasicClient>) -> Value {
    json!({
        "message": "dsasad"
    })
}


pub fn routes() -> Vec<Route> {
    routes![login, verify, index]
}