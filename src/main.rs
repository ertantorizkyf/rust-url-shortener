use crate::handlers::general::hello;

#[macro_use]
extern crate rocket;

mod handlers;
mod responses;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/api/shortener",
            routes![
                hello
            ],
        )
}
