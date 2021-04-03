use config::rocket::RocketManage;

#[macro_use]
extern crate rocket;

mod config;
mod my_domain;
mod stats;

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage_fn(config::stats::configure_prometheus_metrics)
        .manage_fn(my_domain::register_dependencies)
        .mount("/my-domain", my_domain::api::my_domain_routes())
}
