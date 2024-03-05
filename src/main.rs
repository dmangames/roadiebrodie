#[macro_use]
extern crate rocket;
use roadiebrodie::database::mongodb_repo::MongoRepo;
use roadiebrodie::handlers;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;

//I think we need these?
use rocket_oauth2::OAuth2;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .manage(MongoRepo::init())
        .mount("/", handlers::root_handlers())
        .mount("/public", FileServer::from(relative!("static")))
        .mount("/api", handlers::api_handlers())
        .attach(OAuth2::<handlers::GoogleUserInfo>::fairing("google"))
}
