use rocket::local::blocking::Client;
use rocket::{http::Status};

use super::api::HelloWorldResponse;

#[test]
fn should_return_greeting() {
    let client = Client::tracked(crate::rocket()).expect("Rocket client");

    // when
    let response = client.get("/my-domain/hello-world").dispatch();

    // then
    assert_eq!(response.status(), Status::Ok);

    let body: HelloWorldResponse = serde_json::from_str(response.into_string().unwrap().as_str()).unwrap();
    assert_eq!(
        body,
        HelloWorldResponse {
            greeting: String::from("Hello from debug!")
        }
    )
}
