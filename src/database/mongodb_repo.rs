use std::env;
extern crate dotenv;

use dotenv::dotenv;

use crate::models::Pin;
use anyhow::Error;
use mongodb::{
    bson::{self, doc, oid::ObjectId},
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

    pub fn create_pin(&self, mut new_pin: Pin) -> Result<Pin, Error> {
        let pin = self.col.insert_one(&new_pin, None)?;
        let id = match pin.inserted_id {
            bson::Bson::ObjectId(id) => Ok(id.to_hex()),
            _ => Err(anyhow::anyhow!("unexpected db response")),
        }?;
        new_pin.id = Some(id);
        Ok(new_pin)
    }

    pub fn get_pin(&self, id: &str) -> Result<Pin, Error> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": obj_id};
        match self.col.find_one(filter, None)? {
            Some(mut pin) => {
                pin.id = Some(id.to_string());
                Ok(pin)
            }
            None => Err(anyhow::anyhow!("not found")),
        }
    }
}
