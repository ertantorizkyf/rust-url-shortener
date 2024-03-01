# rust-url-shortener

## Description
Rust API using rocket frameworks to shorten a url implementing rand, regex, and redis.

## Project structure
- src/main.rs - Mount all APIs and declare modules
- src/handlers/* - API logic handler
- src/helpers/* - Set of functions to perform specific function, called by handlers
- src/models/* - Data structure
- src/responses/* - Response structure
- assets/URL Shortener.postman_collection.json - API postman collection

## Running the app
- debug mode: `cargo watch -q -c -w src/ -x run`
- release mode: `cargo run --release`
  
