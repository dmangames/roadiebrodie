use std::env;
extern crate dotenv;

use dotenv::dotenv;

use crate::models::Pin;
use anyhow::Error;
use mongodb::{
    bson::{self, doc, oid::ObjectId},
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
        new_pin.db_id = Some(id);
        // Update the newly created pin to set the db_id field.
        self.upsert_pin(new_pin)
    }

    pub fn upsert_pin(&self, pin: Pin) -> Result<Pin, Error> {
        match &pin.db_id {
            None => self.create_pin(pin),
            Some(db_id) => {
                let oid: ObjectId = db_id.parse()?;
                self.col.replace_one(doc! {"_id": oid}, &pin, None)?;
                Ok(pin)
            }
        }
    }

    pub fn get_pin(&self, id: &str) -> Result<Pin, Error> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": obj_id};
        match self.col.find_one(filter, None)? {
            Some(mut pin) => {
                pin.db_id = Some(id.to_string());
                Ok(pin)
            }
            None => Err(anyhow::anyhow!("not found")),
        }
    }

    pub fn get_pins_by_userid(&self, userid: &str) -> Result<Vec<Pin>, Error> {
        let filter = doc! {"user_id": userid};
        let cursor = self.col.find(filter, None)?;
        Ok(cursor.collect::<Result<_, _>>()?)
    }

    pub fn delete_pin(&self, id: &str) -> Result<(), Error> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": obj_id};
        let result = self.col.delete_one(filter, None)?;
        println!("Deleted documents: {}", result.deleted_count);
        println!("Deleted id: {}", id);

        Ok(())
    }
}
/*

use mongodb::{
    bson::doc,
    Client,
    Collection
};
use futures::TryStreamExt;
use serde::{ Deserialize, Serialize };
#[derive(Serialize, Deserialize, Debug)]
struct Restaurant {
    name: String,
    cuisine: String,
}
#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    let client = Client::with_uri_str(uri).await?;
    let my_coll: Collection<Restaurant> = client
        .database("sample_restaurants")
        .collection("restaurants");
    let mut cursor = my_coll.find(
        doc! { "cuisine": "French" },
        None
    ).await?;
    while let Some(doc) = cursor.try_next().await? {
        println!("{:?}", doc);
    }
    Ok(())
}

*/

