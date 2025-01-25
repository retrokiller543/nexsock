use crate::traits::RenderTemplate;
use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};
use nexsock_protocol::commands::list_services::ServiceInfo;
use serde::Serialize;

#[derive(Debug, Serialize, AsRef, AsMut, Deref, DerefMut, From, Into)]
pub struct ServiceBasic(ServiceInfo);

impl ServiceBasic {
    pub fn new(service: ServiceInfo) -> Self {
        Self(service)
    }

    pub fn from_iter(iter: impl IntoIterator<Item = ServiceInfo>) -> Vec<Self> {
        let mut services = Vec::new();

        for service in iter {
            services.push(Self::new(service));
        }

        services
    }
}

impl RenderTemplate for ServiceBasic {
    const TEMPLATE_NAME: &'static str = "service_basic.html";
    const VARIABLE_NAME: &'static str = "service";
}
