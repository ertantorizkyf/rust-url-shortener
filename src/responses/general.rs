use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GeneralResponse {
    pub is_success: bool,
    pub message: String,
    pub data: String
}
