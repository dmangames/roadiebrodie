use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pin {
    pub id: Option<String>,
    pub data: String,
}
