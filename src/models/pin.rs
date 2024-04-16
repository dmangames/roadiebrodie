use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreatePin {
    pub _id: Option<ObjectId>,
    pub user_id: Option<String>,
    pub position: RBPosition,
    pub data: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pin {
    pub _id: ObjectId,
    pub user_id: Option<String>,
    pub position: RBPosition,
    pub data: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RBPosition {
    lat: f64,
    lng: f64,
}
