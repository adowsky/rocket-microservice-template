pub(crate) struct MyDomainFacade {

}

impl MyDomainFacade {
    pub(crate) fn new() -> MyDomainFacade {
        MyDomainFacade {}
    }

    fn say_hello(&self) -> &str {
        "hello!"
    }
}