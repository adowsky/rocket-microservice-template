use std::sync::{Arc, Mutex};
use strum::IntoStaticStr;

#[derive(Debug, Clone, Copy)]
pub(crate) struct HealthStatus {
    pub(crate) health: Health,
}
#[derive(Debug, Clone, Copy, IntoStaticStr)]
pub(crate) enum Health {
    STARTING_UP,
    HEALTHY,
}

pub(crate) trait HealthChecker {
    fn perform_heath_check(&self) -> HealthStatus;
}

#[derive(Clone)]
pub(crate) struct SimpleRocketHealthChecker {
    status: Arc<Mutex<HealthStatus>>,
}

impl SimpleRocketHealthChecker {
    pub(crate) fn new() -> SimpleRocketHealthChecker {
        SimpleRocketHealthChecker {
            status: Arc::new(Mutex::new(HealthStatus {
                health: Health::STARTING_UP,
            })),
        }
    }

    pub(crate) fn started_up(&self) {
        let mut status = self.status.lock().unwrap();
        status.health = Health::HEALTHY
    }
}

impl HealthChecker for SimpleRocketHealthChecker {
    fn perform_heath_check(&self) -> HealthStatus {
        self.status.lock().unwrap().clone()
    }
}
