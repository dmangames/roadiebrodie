#[macro_use]
extern crate rocket;
use mongodb::results::InsertOneResult;
use roadiebrodie::database::mongodb_repo::MongoRepo;
use roadiebrodie::models::Pin;
use rocket::fs::{relative, FileServer};
use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::{json, Json, Value};
use rocket::State;
use rocket_dyn_templates::{context, Template};

//I think we need these?
use anyhow::{anyhow, Context, Error};
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::response::{Debug, Redirect};
use rocket_oauth2::{OAuth2, TokenResponse};

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
fn index(user: Option<User>) -> Template {
    // check if username exists in our cookie
    //  let username = match dbg!(cookies.get_private("username")) {
    //     Some(username) => dbg!(username.value().to_string()),
    //     None => todo!(),
    // };
    // pull all their notes from mongo
    Template::render(
        "index",
        context! {
            field: "value",
            user_name: user.map(|user| user.name),
        },
    )
}

/// User information to be retrieved from the Google People API.
#[derive(serde::Deserialize)]
struct GoogleUserInfo {
    id: String,
    name: String,
    given_name: String,
    family_name: String, //https://stackoverflow.com/questions/7130648/get-user-info-via-google-api
}

#[get("/login/google")]
fn google_login(oauth2: OAuth2<GoogleUserInfo>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["profile"]).unwrap()
}

// google login callback
#[get("/auth/google")]
async fn google_callback(
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

#[get("/pins")]
fn list_pins() -> Result<Value, Status> {
    Ok(json!([Pin {
        id: Some(String::from("1234")),
        user_id: Some(String::from("user")),
        data: String::from("Lorem ipsum"),
    }]))
}

#[get("/pin/<id>")]
fn get_pin(db: &State<MongoRepo>, id: &str) -> Result<Json<Pin>, Status> {
    let pin = db.get_pin(id);
    match pin {
        Ok(pin) => Ok(Json(pin)),
        Err(_) => Err(Status::NotFound),
    }
}

#[get("/user/<userid>")]
fn get_user_pins(db: &State<MongoRepo>, userid: &str) -> Result<Json<Vec<Pin>>, Status> {
    let pin = db.get_pins_by_userid(userid);
    match pin {
        Ok(pin) => Ok(Json(pin)),
        Err(_) => Err(Status::NotFound),
    }
}

#[post("/pin", data = "<input>")]
pub fn create_pin(
    db: &State<MongoRepo>,
    user: User,
    input: Json<Pin>,
) -> Result<Json<Pin>, Status> {
    let data = Pin {
        id: None,
        user_id: Some(user.id),
        data: input.data.to_owned(),
    };
    let pin_detail = db.create_pin(data);
    match pin_detail {
        Ok(pin) => Ok(Json(pin)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/pin/<id>")]
fn delete_pin(id: &str) -> Result<Value, Status> {
    Ok(json!({
        "id": id,
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .manage(MongoRepo::init())
        .mount("/", routes![index, google_callback, google_login])
        .mount("/public", FileServer::from(relative!("static")))
        .mount(
            "/api",
            routes![list_pins, get_pin, create_pin, delete_pin, get_user_pins],
        )
        .attach(OAuth2::<GoogleUserInfo>::fairing("google"))
}
