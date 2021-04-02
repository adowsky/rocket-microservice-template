#[macro_use]
extern crate rocket;

mod config;
mod my_domain;


#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .mount("/my-domain", my_domain::api::my_domain_routes())
}
