
use super::rocket;
use rocket::local::blocking::Client;
use rocket::http::Status;
#[test]
fn index() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(uri!(super::index)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Hello, world!");
}
//#[test]
//fn non_existant_items_return_empty_string() {
//    let client = Client::tracked(rocket()).expect("valid rocket instance");
//    let mut response = client.get(uri!(super::hello)).dispatch();
//    assert_eq!(response.status(), Status::Ok);
//    assert_eq!(response.into_string().unwrap(), "Hello, world!");
//}
