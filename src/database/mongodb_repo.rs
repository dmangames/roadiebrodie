use std::env;
extern crate dotenv;

use dotenv::dotenv;

use crate::models::{Pin, Trip};
use anyhow::Error;
use mongodb::{
    bson::{self, doc, oid::ObjectId},
    sync::{Client, Collection},
};

pub struct MongoRepo {
    pins: Collection<Pin>,
    trips: Collection<Trip>,
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
        let pins = db.collection("Pin");
        let trips = db.collection("Trip");
        MongoRepo { pins, trips }
    }

    pub fn create_pin(&self, new_pin: Pin) -> Result<Pin, Error> {
        let pin = self.pins.insert_one(&new_pin, None)?;
        let id = match pin.inserted_id {
            bson::Bson::ObjectId(id) => Ok(id.to_hex()),
            _ => Err(anyhow::anyhow!("unexpected db response")),
        }?;
    }

    pub fn upsert_pin(&self, pin: Pin) -> Result<Pin, Error> {
        self.pins.replace_one(doc! {"_id": pin._id}, &pin, None)?;
        Ok(pin)
    }

    pub fn get_pin(&self, id: &str) -> Result<Pin, Error> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": obj_id};
        self.pins.find_one(filter, None)?.ok_or_else(|| anyhow::anyhow!("not found"))
    }

    pub fn get_pins_by_userid(&self, userid: &str) -> Result<Vec<Pin>, Error> {
        let filter = doc! {"user_id": userid};
        let cursor = self.pins.find(filter, None)?;
        Ok(cursor.collect::<Result<_, _>>()?)
    }

    pub fn delete_pin(&self, id: &str) -> Result<(), Error> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": obj_id};
        let result = self.pins.delete_one(filter, None)?;
        println!("Deleted documents: {}", result.deleted_count);
        println!("Deleted id: {}", id);

        Ok(())
    }

    pub fn create_trip(&self, new_trip: Trip) -> Result<Trip, Error> {
        todo!()
    }

    pub fn get_trip(&self, id: &str) -> Result<Trip, Error> {
        todo!()
    }

    pub fn get_trips_by_userid(&self, userid: &str) -> Result<Vec<Pin>, Error> {
        todo!()
    }

    pub fn delete_trip(&self, id: &str) -> Result<(), Error> {
        todo!()
    }
}
