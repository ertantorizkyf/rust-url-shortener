use crate::responses::general::GeneralResponse;

use rocket::{
    get, 
    http::Status,
    serde::json::Json
};

#[get("/hello")]
pub fn hello() -> Result<Json<GeneralResponse>, Status> {
    let json_response = GeneralResponse {
        is_success: true,
        message: "success".to_string(),
        data: "Hello World!".to_string()
    };

    Ok(Json(json_response))
}