use crate::handlers::general::hello;
use crate::handlers::shortener::shorten_url;

#[macro_use]
extern crate rocket;

mod handlers;
mod helpers;
mod models;
mod responses;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/api/shortener",
            routes![
                hello,
                shorten_url
            ],
        )
}
