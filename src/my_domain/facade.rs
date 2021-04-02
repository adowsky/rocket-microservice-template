use super::config::{MyDomainProperties};

pub(crate) struct MyDomainFacade {
    config: MyDomainProperties,
}

impl MyDomainFacade {
    pub(crate) fn new(config: MyDomainProperties) -> MyDomainFacade {
        MyDomainFacade { config }
    }

    pub(crate) fn say_hello(&self) -> &str {
        self.config
            .greeting
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("Default greeting")
    }
}
