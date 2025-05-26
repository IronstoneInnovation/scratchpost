#[macro_use]
extern crate rocket;

use rocket::serde::{Deserialize, json::Json};
use rocket::State;
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
fn post_item(simple_cache: &State<SimpleCache>, item: Json<Item<'_>>) -> &'static str {
    // TODO: Don't allow empty "" values
    "Post"
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
        .manage(new_simple_cache())
}
