#[macro_use]
extern crate rocket;

use std::sync::Mutex;
use rocket::serde::{Deserialize, json::Json};
use rocket::State;
use rocket::http::Status;
use rocket::response::{content, status};
use scratchpost::{new_simple_cache, SimpleCache};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]

struct Item<'r> {
    key: &'r str,
    value: &'r str
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/item", format = "application/json", data = "<item>")]
fn post_item(simple_cache: &State<Mutex<SimpleCache>>, item: Json<Item<'_>>) -> Status {
    // TODO: Don't allow empty "" values
    let mut cache = simple_cache.lock().expect("SimpleCache lock poisoned");
    cache.push(item.key.to_string(), item.value.to_string());
    Status::Ok
}

#[get("/item")]
fn get_item() -> &'static str {
    "Get"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/item", routes![post_item])
        .mount("/item", routes![get_item])
        .manage(Mutex::new(new_simple_cache()))
}
