use crate::database::mongodb_repo::MongoRepo;
use crate::models::{Pin, CreatePin};
use mongodb::bson::oid::ObjectId;
use rocket::serde::json::Json;
use rocket::{delete, get, post, routes};
use rocket::{Route, State};
use tracing::{info};

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
    println!("getting pins!");
    let pins_result = db.get_pins_by_userid(&user.id);
    match pins_result {
        Ok(pins) => Ok(Json(pins)),
        Err(_) => Err(Status::NotFound),
    }
}

#[post("/pin", data = "<input>")]
fn create_pin(db: &State<MongoRepo>, user: User, input: Json<CreatePin>) -> Result<Json<Pin>, Status> {
    let data = Pin {
        _id: input.0._id.unwrap_or_else(||ObjectId::new()),
        user_id: Some(user.id),
        position: input.0.position,
        data: input.0.data
    };
    println!("saving pin: {data:?}");
    let p = if input.0._id.is_some() {
        db.upsert_pin(data)
    } else {
        db.create_pin(data)
    };
    match p {
        Ok(p) => Ok(Json(p)),
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
