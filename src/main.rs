#[macro_use]
extern crate rocket;

use rocket::serde::{Deserialize, json::Json};

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
fn post_item(item: Json<Item<'_>>) -> &'static str {
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
}