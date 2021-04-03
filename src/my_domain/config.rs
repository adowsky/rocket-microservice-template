use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct MyDomainProperties {
    pub greeting: Option<String>,

}

#[derive(Deserialize)]
pub(crate) struct MyDomainConfig {
    pub(crate) my_domain: MyDomainProperties
}
