use crate::infrastructure::health::{HealthChecker, SimpleRocketHealthChecker};
use rocket::fairing::{AdHoc};
use rocket::{get, Rocket, State};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct HealthStatusResponse {
    pub(crate) status: String,
}

#[get("/health")]
fn get_health_status(
    health_checker: State<SimpleRocketHealthChecker>,
) -> Json<HealthStatusResponse> {
    let status: &'static str = health_checker.perform_heath_check().health.into();
    Json(HealthStatusResponse {
        status: status.to_string(),
    })
}

pub(crate) fn configure_status_endpoints(rocket: Rocket) -> Rocket {
    let health_checker = SimpleRocketHealthChecker::new();
    rocket
        .manage(health_checker)
        .attach(AdHoc::on_launch("HealthChecker::on_launch", |launched_rocket| {
            launched_rocket.state::<SimpleRocketHealthChecker>().unwrap().started_up();
        }))
        .mount("/status", routes![get_health_status])
}