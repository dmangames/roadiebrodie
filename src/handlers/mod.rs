mod auth;
mod pin;
mod trip;

use crate::database::mongodb_repo::MongoRepo;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::{get, routes};
use rocket::{Route, State};
use rocket_dyn_templates::{context, Template};

use anyhow::{anyhow, Error};
use rocket::http::Status;

pub use auth::GoogleUserInfo;

pub fn root_handlers() -> Vec<Route> {
    routes![
        index,
        auth::google_callback,
        auth::google_login,
        about,
        contact,
        services,
        pricing,
    ]
}

pub fn api_handlers() -> Vec<Route> {
    [pin::handlers(), trip::handlers()]
        .into_iter()
        .flat_map(std::convert::identity)
        .collect()
}

#[derive(Debug)]
pub struct User {
    id: String,
    name: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let id = request
            .cookies()
            .get_private("user_id")
            .map(|cookie| cookie.value().to_string());
        let name = request
            .cookies()
            .get_private("user_name")
            .map(|cookie| cookie.value().to_string());
        match (id, name) {
            (Some(id), Some(name)) => Outcome::Success(User { id, name }),
            _ => Outcome::Error((Status::Unauthorized, anyhow!("not logged in"))),
        }
    }
}
#[get("/")]
fn index(maybe_user: Option<User>, db: &State<MongoRepo>) -> Template {
    let user_name: Option<&str> = match maybe_user {
        Some(ref user) => Some(user.name.as_str()),
        None => None,
    };

    Template::render(
        "index",
        context! {
            user_name: user_name,
        },
    )
}

#[get("/pricing")]
fn pricing(maybe_user: Option<User>, db: &State<MongoRepo>) -> Template {
    let user_name: Option<&str> = match maybe_user {
        Some(ref user) => Some(user.name.as_str()),
        None => None,
    };

    Template::render(
        "pricing",
        context! {
            user_name: user_name,
        },
    )
}

#[get("/about")]
fn about(maybe_user: Option<User>, db: &State<MongoRepo>) -> Template {
    let user_name: Option<&str> = match maybe_user {
        Some(ref user) => Some(user.name.as_str()),
        None => None,
    };

    Template::render(
        "about",
        context! {
            user_name: user_name,
        },
    )
}

#[get("/services")]
fn services(maybe_user: Option<User>, db: &State<MongoRepo>) -> Template {
    let user_name: Option<&str> = match maybe_user {
        Some(ref user) => Some(user.name.as_str()),
        None => None,
    };

    Template::render(
        "services",
        context! {
            user_name: user_name,
        },
    )
}

#[get("/contact")]
fn contact(maybe_user: Option<User>, db: &State<MongoRepo>) -> Template {
    let user_name: Option<&str> = match maybe_user {
        Some(ref user) => Some(user.name.as_str()),
        None => None,
    };

    Template::render(
        "contact",
        context! {
            user_name: user_name,
        },
    )
}
