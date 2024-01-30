use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pin {
    pub db_id: Option<String>,
    pub user_id: Option<String>,
    pub position: RBPosition,
    pub data: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RBPosition {
    lat: f64,
    lng: f64,
}
