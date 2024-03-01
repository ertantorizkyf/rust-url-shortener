use crate::{
    helpers::shortener::{
        url_revealer,
        url_shortener
    },
    helpers::validation::url_validator,
    models::shortener::ShortenerBody,
    responses::general::GeneralResponse
};

use std::env;
use rocket::{
    post,
    http::Status,
    response::status::Custom, 
    serde::json::Json
};

#[post("/", data="<body>")]
pub fn shorten_url(
    body: Json<ShortenerBody>
) -> Result<Json<GeneralResponse>, Custom<Json<GeneralResponse>>> {
    let url = body.url.clone();
    let is_valid_url = url_validator(url.clone());
    if !is_valid_url {
        let err_response = GeneralResponse {
            is_success: false,
            message: format!("{} is not a valid URL!!!", url),
            data: "".to_string()
        };

        return Err(Custom(Status::BadRequest, Json(err_response)));
    }

    let base_url = match env::var_os("BASE_URL") {
        Some(v) => v.into_string().unwrap(),
        None => "http://localhost:8000/".to_string()
    };
    let shortened_url = url_shortener(url.clone());
    
    let json_response = GeneralResponse {
        is_success: true,
        message: "URL successfully shortened".to_string(),
        data: format!("{}api/shortener/reveal/{}", base_url, shortened_url)
    };

    Ok(Json(json_response))
}

#[get("/reveal/<shortened_url>")]
pub async fn reveal_url(
    shortened_url: String
) -> Result<Json<GeneralResponse>, Custom<Json<GeneralResponse>>> {
    let revealed_url = url_revealer(shortened_url.clone());
    if revealed_url.is_empty() {
        let base_url = match env::var_os("BASE_URL") {
            Some(v) => v.into_string().unwrap(),
            None => "http://localhost:8000/".to_string()
        };
        
        let err_response = GeneralResponse {
            is_success: false,
            message: format!("{}api/shortener/reveal/{} is not a shortened URL!!!", base_url, shortened_url),
            data: "".to_string()
        };

        return Err(Custom(Status::NotFound, Json(err_response)));
    }

    let json_response = GeneralResponse {
        is_success: true,
        message: "URL successfully revealed".to_string(),
        data: revealed_url
    };

    Ok(Json(json_response))
}
