
use super::rocket;
use rocket::local::blocking::Client;
use rocket::http::{ContentType, Status};

#[test]
/// Test GET / returns the correct response.
fn index() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Feed me!");
}

#[test]
/// Test that we can POST a new item to /item and read it back using GET /item/<key>
fn post_and_get_item() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");

    // POST new item
    let response = client.post("/item")
        .header(ContentType::JSON)
        .body(r##"{
            "key": "testkey",
            "value": "testvalue"
        }"##)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "{\"msg\": \"ok\"}");

    // GET item
    let response = client.get("/item/testkey")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "{\"value\": \"testvalue\"}");
}

#[test]
/// Test that getting a non-existant item returns an empty string value.
fn get_nonexistant_item_returns_empty_string() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");

    // GET item
    let response = client.get("/item/doesnotexist")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "{\"value\": \"\"}");
}
