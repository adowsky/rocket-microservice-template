use rocket::Rocket;

pub(crate) trait RocketManage {
    fn manage_fn(self, registrator: fn(Rocket) -> Rocket) -> Self;
}

impl RocketManage for Rocket {
    fn manage_fn(self, registrator: fn(Rocket) -> Rocket) -> Self {
        registrator(self)
    }
}
