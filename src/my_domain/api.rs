use rocket::{get, Route};
use rocket_contrib::json::Json;

#[get("/hello-world")]
fn hello_world() -> Json<String> {
    Json("hello world!".to_string())
}

pub(crate) fn my_domain_routes() -> Vec<Route> {
    routes![hello_world]
}