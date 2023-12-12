use anyhow::{Context, Error};
use hyper::{
    body,
    header::{ACCEPT, AUTHORIZATION, USER_AGENT},
    Body, Client, Request,
};
use rocket::fairing::{AdHoc, Fairing};
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::{Debug, Redirect};
use rocket_oauth2::{OAuth2, TokenResponse};

/// User information to be retrieved from the GitHub API.
#[derive(serde::Deserialize)]
struct GoogleUserInfo {
    #[serde(default)]
    name: String,
}

/// Rocket fairing for managing the GitHub OAuth2 flow
///
/// The third argument passed into OAuth2::fairing is the
/// config_name which must match the key used in Rocket.toml
/// to specify the custom provider attributes.
pub fn fairing() -> impl Fairing {
    AdHoc::on_ignite("Github OAuth2", |rocket| async {
        rocket
            .mount("/", rocket::routes![github_login, post_install_callback])
            .attach(OAuth2::<GoogleUserInfo>::fairing("google"))
    })
}