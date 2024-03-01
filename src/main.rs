use crate::handlers::shortener::{
    shorten_url,
    reveal_url
};
use helpers::catcher::not_found;

#[macro_use]
extern crate rocket;

mod handlers;
mod helpers;
mod models;
mod responses;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount(
            "/api/shortener",
            routes![
                shorten_url,
                reveal_url
            ],
        )
}
