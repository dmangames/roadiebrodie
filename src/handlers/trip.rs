use crate::database::mongodb_repo::MongoRepo;
use crate::models::Trip;
use rocket::serde::json::Json;
use rocket::{delete, get, post, routes};
use rocket::{Route, State};

use crate::handlers::User;
use rocket::http::Status;

#[get("/trip/<id>")]
fn get_trip(db: &State<MongoRepo>, id: &str) -> Result<Json<Trip>, Status> {
    todo!()
}

#[get("/trips")]
fn get_trips(user: User, db: &State<MongoRepo>) -> Result<Json<Vec<Trip>>, Status> {
    todo!()
}

#[post("/trip", data = "<input>")]
fn create_trip(db: &State<MongoRepo>, user: User, input: Json<Trip>) -> Result<Json<Trip>, Status> {
    todo!()
}

#[delete("/trip/<id>")]
fn delete_trip(db: &State<MongoRepo>, id: &str) -> Result<(), Status> {
    todo!()
}

pub fn handlers() -> Vec<Route> {
    routes![get_trip, get_trips, create_trip, delete_trip]
}
