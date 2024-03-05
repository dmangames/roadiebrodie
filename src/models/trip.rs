use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Trip {
    pub db_id: Option<String>,
    pub user_id: Option<String>,
    pub start: RBPosition,
    pub end: RBPosition,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RBPosition {
    lat: f64,
    lng: f64,
}
