use rocket::Rocket;

pub(crate) trait RocketManage {
    fn manage_dependencies(self, registrator: fn(Rocket) -> Rocket) -> Self;
}

impl RocketManage for Rocket {
    fn manage_dependencies(self, registrator: fn(Rocket) -> Rocket) -> Self {
        registrator(self)
    }
}
