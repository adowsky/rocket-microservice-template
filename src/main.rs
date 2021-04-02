use config::rocket::RocketManage;

#[macro_use]
extern crate rocket;

mod config;
mod my_domain;

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage_dependencies(my_domain::register_dependencies)
        .mount("/my-domain", my_domain::api::my_domain_routes())
}
