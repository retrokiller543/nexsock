use crate::traits::RenderTemplate;
use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};
use nexsock_protocol::commands::service_status::ServiceStatus;
use serde::Serialize;

#[derive(Debug, Serialize, AsRef, AsMut, Deref, DerefMut, From, Into)]
pub struct ServiceStatusView(ServiceStatus);

impl ServiceStatusView {
    pub fn new(status: ServiceStatus) -> Self {
        Self(status)
    }
}

impl RenderTemplate for ServiceStatusView {
    const TEMPLATE_NAME: &'static str = "service_page.html";
    const VARIABLE_NAME: &'static str = "service";
}
