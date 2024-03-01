use std::env;
use rand::distributions::{
    DistString, 
    Alphanumeric
};
use redis::Commands;

pub fn url_shortener(url: String) -> String {
    let redis_url = match env::var_os("REDIS_URL") {
        Some(v) => v.into_string().unwrap(),
        None => "redis://127.0.0.1:6379".to_string()
    };
    let mut client = redis::Client::open(redis_url).unwrap();

    let mut is_new_shortened_url = false;
    let mut shortened_url = String::new();
    while !is_new_shortened_url {
        shortened_url = Alphanumeric.sample_string(&mut rand::thread_rng(), 5);

        // check if generated key has been used before
        let redis_search_val = client.get(&shortened_url).unwrap_or("".to_string());
        if redis_search_val.is_empty() {
            match client.set(&shortened_url, &url) {
                Ok(()) => println!("{} SAVED TO REDIS", &shortened_url),
                Err(e) => println!("REDIS SET ERR: {:?}", e)
            };
            is_new_shortened_url = true;
        }
    }
    
    shortened_url
}

pub fn url_revealer(shortened_url: String) -> String {
    let redis_url = match env::var_os("REDIS_URL") {
        Some(v) => v.into_string().unwrap(),
        None => "redis://127.0.0.1:6379".to_string()
    };
    let mut client = redis::Client::open(redis_url).unwrap();

    let redis_search_val = client.get(&shortened_url).unwrap_or("".to_string());
    
    redis_search_val
}
