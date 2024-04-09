use rocket::get;

use anyhow::{Context, Error};
use reqwest::header::AUTHORIZATION;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::{Debug, Redirect};
use rocket_oauth2::{OAuth2, TokenResponse};
use rocket::serde::json::Json;

use super::User;

/// User information to be retrieved from the Google People API.
#[derive(serde::Deserialize)]
pub struct GoogleUserInfo {
    id: String,
    name: String,
    given_name: String,
    family_name: String, //https://stackoverflow.com/questions/7130648/get-user-info-via-google-api
}

#[get("/login/google")]
pub fn google_login(oauth2: OAuth2<GoogleUserInfo>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["profile"]).unwrap()
}

// google login callback
#[get("/auth/google")]
pub async fn google_callback(
    token: TokenResponse<GoogleUserInfo>,
    cookies: &CookieJar<'_>,
) -> Result<Redirect, Debug<Error>> {
    // Use the token to retrieve the user's Google account information.
    let user_info: GoogleUserInfo = reqwest::Client::builder()
        .build()
        .context("failed to build reqwest client")?
        .get("https://www.googleapis.com/oauth2/v1/userinfo?alt=json")
        .header(AUTHORIZATION, format!("Bearer {}", token.access_token()))
        .send()
        .await
        .context("failed to complete request")?
        .json()
        .await
        .context("failed to deserialize response")?;

    let add_cookie = |k, v| {
        cookies.add_private(Cookie::build((k, v)).same_site(SameSite::Lax));
    };
    // Set a private cookie with the user's name, and redirect to the home page.
    add_cookie("user_id", user_info.id);
    add_cookie("user_name", user_info.given_name);
    Ok(Redirect::to("/"))
}

#[get("/user")]
pub fn user(user: User) -> Json<User> {
    Json(user)
}