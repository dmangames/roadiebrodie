use std::env;
extern crate dotenv;

use dotenv::dotenv;

use crate::models::Pin;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};

pub struct MongoRepo {
    col: Collection<Pin>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<Pin> = db.collection("Pin");
        MongoRepo { col }
    }

    pub fn create_pin(&self, new_pin: Pin) -> Result<InsertOneResult, Error> {
        let pin = self
            .col
            .insert_one(new_pin, None)
            .ok()
            .expect("Error creating pin");
        Ok(pin)
    }
}
