//! # scratchpost
//! 
//! _A simple key/value cache server built in Rust._
//! 
//! ## Installation
//! 
//! Install scratchpost with cargo:
//! 
//! ```bash
//! $ cargo install scratchpost
//! ```
//! 
//! ## Settings
//! 
//! There's only one setting available currently, the maximum size of the cache.  This is controlled by setting the SCRATCHPOST_MAX_ITEMS environment variable; if this is not set the default value of 1000 is used.
//! 
//! ## Usage
//! 
//! Start scratchpost:
//! 
//! ```bash
//! $ scratchpost
//! ```
//! 
//! When launch Rocket will print the following (your port number may differ):
//! 
//! ```
//! Rocket has launched from http://127.0.0.1:8000
//! ```
//! 
//! ### Endpoints
//! 
//! #### `GET /`
//! 
//! Returns http status 200 and the message "Feed me!".
//! 
//! #### `POST /item`
//! 
//! Stores an item (key/value pair) in the cache.  The item is included in the request `body` as a JSON object, like this:
//! 
//! ```json
//! {
//!     "key": "string",
//!     "value": "string"
//! }
//! ```
//! 
//! Note that an empty string value will not be accepted and status 400 is returned.
//! 
//! If successful, http status 200 is returned.
//! 
//! #### `GET /item/<key>`
//! 
//! Retrieves the value of an item from the cache.  If the item does not exist an empty string is returned.
//! 
//! ## License
//! 
//! MIT
#[cfg(test)] mod api_tests;
#[cfg(test)] mod cache_tests;

#[macro_use]
extern crate rocket;

use std::sync::Mutex;
use rocket::serde::{Deserialize, json::Json};
use rocket::State;
use rocket::http::{Status, ContentType};
mod simple_cache;
use simple_cache::SimpleCache;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]

struct Item<'r> {
    key: &'r str,
    value: &'r str
}

#[get("/")]
fn index() -> &'static str {
    "Feed me!"
}

#[post("/item", format = "application/json", data = "<item>")]
fn post_item(simple_cache: &State<Mutex<SimpleCache>>, item: Json<Item<'_>>) -> (Status, (ContentType, String)) {
    // Don't allow empty "" values
    if item.value == "".to_string() {
        return (Status::BadRequest, (ContentType::JSON, "{ \"msg\": \"empty values are not allowed\" }".to_string()))
    }
    // Otherwise push key/value to cache
    let mut cache = simple_cache.lock().expect("cache lock poisoned during POST");
    cache.push(item.key.to_string(), item.value.to_string());
    (Status::Ok, (ContentType::JSON, "{\"msg\": \"ok\"}".to_string()))
}


#[get("/item/<key>")]
fn get_item(simple_cache: &State<Mutex<SimpleCache>>, key: &str) -> (Status, (ContentType, String)) {
    let mut cache = simple_cache.lock().expect("cache lock poisoned during GET");
    let body = format!("{{\"value\": \"{}\"}}", cache.get(key.to_string()));
    (Status::Ok, (ContentType::JSON, body))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, post_item, get_item])
        .manage(Mutex::new(simple_cache::new()))
}
