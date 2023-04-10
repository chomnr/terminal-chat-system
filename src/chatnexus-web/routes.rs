use oauth2::{AuthorizationCode, reqwest::async_http_client, TokenResponse};
use rocket::{Route, routes, get, response::Redirect, State, post, http::CookieJar};
use serde_json::{json, Value};

use crate::oauth2::OAuth2;

#[get("/?<code>")]
async fn index(code: String, oauth2: &State<OAuth2>) -> Value {
    let result = oauth2.exchange_auth_code(code).await;
    let data = oauth2.post_discord(result.access_token()).await;
    json!({
        "id": data.id(),
        "username": data.username(),
    })
}

#[get("/login")]
fn login(oauth2: &State<OAuth2>) -> Redirect {
    Redirect::to(oauth2.authorize_url())
}


pub fn routes() -> Vec<Route> {
    routes![index, login]
}

/*

#[get("/login")]
fn login(oauth2: &State<OAuth2>) -> Redirect {
    Redirect::to(oauth2.authorize_url())
}

#[get("/?<code>")]
fn index(jar: &CookieJar, code: String, oauth2: &State<OAuth2>) -> Value {
    /*
    let code = oauth2.exchange_code(AuthorizationCode::new(code));
    json!({
        "message": code.tr
    })
    */
    todo!()
}

 */


/*
json!({
        "message": code
    })

#[get("/verify")]
fn verify(oauth2: &State<BasicClient>) -> Value {
    json!({
        "message": "dsasad"
    })
}
 */

/*
#[get("/verify")]
fn verify(oauth2: &State<BasicClient>) -> Value {
    json!({
        "message": "dsasad"
    })
}
*/