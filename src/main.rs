#[macro_use]
extern crate rocket;
use mongodb::results::InsertOneResult;
use roadiebrodie::database::mongodb_repo::MongoRepo;
use roadiebrodie::models::Pin;
use rocket::fs::{relative, FileServer};
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::State;
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            field: "value",
        },
    )
}

#[get("/pins")]
fn list_pins() -> Result<Value, Status> {
    Ok(json!([Pin {
        id: Some(String::from("1234")),
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

#[post("/pin", data = "<input>")]
pub fn create_pin(db: &State<MongoRepo>, input: Json<Pin>) -> Result<Json<Pin>, Status> {
    let data = Pin {
        id: None,
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
        .mount("/", routes![index])
        .mount("/public", FileServer::from(relative!("static")))
        .mount("/api", routes![list_pins, get_pin, create_pin, delete_pin])
}
