use crate::responses::general::GeneralResponse;

use rocket::{
    Request,
    serde::json::Json
};

#[catch(404)]
pub fn not_found(req: &Request) -> Json<GeneralResponse> {
    let uri = req.uri().to_string();
    let response = GeneralResponse { 
        is_success: false,
        message: format!("PAGE {} COULD NOT BE FOUND!!!", uri),
        data: "".to_string()
    };

    Json(response)
}
