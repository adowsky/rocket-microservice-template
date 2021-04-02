pub(crate) mod api;
pub(crate) mod facade;

use rocket::Rocket;


pub(crate) fn register_dependencies(rocket: Rocket) -> Rocket {
    rocket.manage(facade::MyDomainFacade::new())
}
