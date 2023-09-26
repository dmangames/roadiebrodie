#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world! I am adding some random text here since I can't figure out how to add an image\nGoodbye, world!"
    
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

