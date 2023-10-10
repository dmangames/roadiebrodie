#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use rocket::fs::{FileServer, relative};
use rocket::serde::json::{json, Value};
use rocket::{http::Status};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        field: "value",
    })
}

#[get("/hello")]
fn hello() -> Result<Value, Status> {
  Ok(json!({
    "key": "value",
    "array": [1, 2, 3, 4]
  }))
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/public", FileServer::from(relative!("static")))
        .mount("/api", routes![hello])
}
