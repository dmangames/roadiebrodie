#[macro_use]
extern crate rocket;
use mongodb::results::InsertOneResult;
use roadiebrodie::database::mongodb_repo::MongoRepo;
use roadiebrodie::models::Pin;
use rocket::fs::{relative, FileServer};
use rocket::serde::json::{json, Json, Value};
use rocket::State;
use rocket_dyn_templates::{context, Template};

//I think we need these?
use rocket_oauth2::{OAuth2, TokenResponse};
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::response::{Debug, Redirect};
use anyhow::{Error, Context};

#[get("/")]
fn index(cookies: &CookieJar<'_>) -> Template {
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
        },
    )
}

/// User information to be retrieved from the Google People API.
#[derive(serde::Deserialize)]
struct GoogleUserInfo {
    id: String,
    name: String,
    given_name: String,
    family_name: String
    //https://stackoverflow.com/questions/7130648/get-user-info-via-google-api
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

    let real_id: String = dbg!(user_info.id);
    let real_name = dbg!(user_info.given_name);

    // Set a private cookie with the user's name, and redirect to the home page.
    cookies.add_private(
        Cookie::build("username", real_id.to_string())
            .same_site(SameSite::Lax)
            .finish()
    );
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
pub fn create_pin(db: &State<MongoRepo>, cookies: &CookieJar<'_>, input: Json<Pin>) -> Result<Json<Pin>, Status> {
    let username = match dbg!(cookies.get_private("username")) {
        Some(username) => dbg!(username.value().to_string()),
        None => todo!(),
    };

    let data = Pin {
        id: None,
        user_id: Some(username),
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
        .mount("/api", routes![list_pins, get_pin, create_pin, delete_pin, get_user_pins])
        .attach(OAuth2::<GoogleUserInfo>::fairing("google"))
}
