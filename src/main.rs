#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use rocket::fs::{FileServer, relative};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        field: "value",
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/public", FileServer::from(relative!("static")))
}