use super::facade::MyDomainFacade;
use rocket::{get, Route, State};
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize)]
struct HelloWorldResponse {
    greeting: String,
}

#[get("/hello-world")]
fn hello_world(my_domain: State<MyDomainFacade>) -> Json<HelloWorldResponse> {
    Json(HelloWorldResponse {
        greeting: my_domain.say_hello().to_string(),
    })
}

pub(crate) fn my_domain_routes() -> Vec<Route> {
    routes![hello_world]
}
