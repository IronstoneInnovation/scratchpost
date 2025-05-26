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
