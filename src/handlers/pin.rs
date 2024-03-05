use crate::database::mongodb_repo::MongoRepo;
use crate::models::Pin;
use rocket::serde::json::Json;
use rocket::{delete, get, post, routes};
use rocket::{Route, State};

use crate::handlers::User;
use rocket::http::Status;

#[get("/pin/<id>")]
fn get_pin(db: &State<MongoRepo>, id: &str) -> Result<Json<Pin>, Status> {
    let pin = db.get_pin(id);
    match pin {
        Ok(pin) => Ok(Json(pin)),
        Err(_) => Err(Status::NotFound),
    }
}

#[get("/pins")]
fn get_user_pins(user: User, db: &State<MongoRepo>) -> Result<Json<Vec<Pin>>, Status> {
    let pin = db.get_pins_by_userid(&user.id);
    match pin {
        Ok(pin) => Ok(Json(pin)),
        Err(_) => Err(Status::NotFound),
    }
}

#[post("/pin", data = "<input>")]
fn create_pin(db: &State<MongoRepo>, user: User, input: Json<Pin>) -> Result<Json<Pin>, Status> {
    let data = Pin {
        user_id: Some(user.id),
        ..input.0
    };
    let pin_detail = db.upsert_pin(data);
    match pin_detail {
        Ok(pin) => Ok(Json(pin)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/delete_pin/<id>")]
fn delete_pin(db: &State<MongoRepo>, id: &str) -> Result<(), Status> {
    let pin = db.delete_pin(id);
    match pin {
        Ok(pin) => Ok(()),
        Err(_) => Err(Status::NotFound),
    }
}

pub fn handlers() -> Vec<Route> {
    routes![get_pin, create_pin, delete_pin, get_user_pins]
}
