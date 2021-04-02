pub(crate) mod api;
pub(crate) mod facade;
mod config;

#[cfg(test)]
mod integration_tests;

use rocket::Rocket;


pub(crate) fn register_dependencies(rocket: Rocket) -> Rocket {
    let config = rocket.figment().extract::<config::MyDomainConfig>()
    .expect("MyDomainConfig to exist in configuration");
    rocket.manage(facade::MyDomainFacade::new(config.my_domain))
}
