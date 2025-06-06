use crate::components::service_basic::ServiceBasic;
use crate::traits::RenderTemplate;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ServicesList {
    pub services: Vec<ServiceBasic>,
}

impl ServicesList {
    pub fn new(services: Vec<ServiceBasic>) -> Self {
        Self { services }
    }
}

impl RenderTemplate for ServicesList {
    const TEMPLATE_NAME: &'static str = "services_list.html";
    const VARIABLE_NAME: &'static str = "services_list";
}
