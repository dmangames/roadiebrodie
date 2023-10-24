#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket::http::Status;
use rocket::serde::json::{json, Value};
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            field: "value",
        },
    )
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Pin {
    id: String,
    data: String,
}

#[get("/pins")]
fn list_pins() -> Result<Value, Status> {
    Ok(json!([Pin {
        id: String::from("1234"),
        data: String::from("Lorem ipsum"),
    }]))
}

#[get("/pin/<id>")]
fn get_pin(id: &str) -> Result<Value, Status> {
    Ok(json!(Pin{
        id: id.into(),
        data: String::from("Lorem ipsum"),
    }))
}

#[post("/pin", data = "<input>")]
fn create_pin(input: &str) -> Result<Value, Status> {
    Ok(json!(Pin{
        id: "1234".into(),
        data: input.into(),
    }))
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
        .mount("/", routes![index])
        .mount("/public", FileServer::from(relative!("static")))
        .mount("/api", routes![list_pins, get_pin, create_pin, delete_pin])
}
