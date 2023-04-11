use oauth2::{AuthorizationCode, reqwest::async_http_client, TokenResponse};
use rocket::{Route, routes, get, response::Redirect, State, post, http::{CookieJar, Cookie}};
use serde_json::{json, Value};

use crate::oauth2::OAuth2;

#[get("/?<code>")]
async fn index(code: String, oauth2: &State<OAuth2>, jar: &CookieJar<'_>) -> Result<Redirect, Value> {
    let result = oauth2.exchange_auth_code(code).await;
    let data = oauth2.post_discord(result.access_token()).await;
    if jar.get("sid").is_some() {
        return Ok(Redirect::to("/verify"))
    }
    match data {
        Ok(user) => {
            let cookie = Cookie::build("sid", serde_json::to_string(&user).unwrap())
                .same_site(rocket::http::SameSite::None)
                .secure(false)
                .finish();
            jar.add(cookie);
            Ok(Redirect::to("/verify"))
        },
        Err(_) => {
            return Err(
                json!({
                    "message": "failed to authorize with the intermediator"
                })
            )
        },
    }
}

#[get("/login")]
fn login(oauth2: &State<OAuth2>) -> Redirect {
    Redirect::to(oauth2.authorize_url())
}


#[get("/verify")]
fn verify(jar: &CookieJar<'_>, oauth2: &State<OAuth2>) -> Value {
    //Redirect::to(oauth2.authorize_url())
    if jar.get("sid").is_some() {
        json!({
            "message": "in development"
        })
    } else {
        json!({
            "message": "No permission...."
        })
    }
}

#[post("/identity/check")]
fn identitycheck(jar: &CookieJar<'_>, oauth2: &State<OAuth2>) -> Value {
    //Redirect::to(oauth2.authorize_url())
    json!({
        "message": "Check identity"
    })
}

pub fn routes() -> Vec<Route> {
    routes![index, login, verify, identitycheck]
}

 //let cookie = Cookie::build("name", serde_json::to_string(&data).unwrap());
    
    /*
    json!({
        "id": data.id(),
        "username": data.username(),
    })
    */

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