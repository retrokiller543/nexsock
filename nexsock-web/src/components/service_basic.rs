use nexsock_protocol::commands::list_services::ServiceInfo;
use rust_html::{rhtml, Render, Template};

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

impl Render for ServiceBasic {
    fn render(&self) -> Template {
        let ServiceInfo {
            name,
            state,
            port,
            has_dependencies,
        } = &self.0;

        rhtml! {r#"
        <div class="service">
            <div>
                <strong>{name}</strong>
                <span style="float: right">{state}</span>
            </div>
            <div style="margin-top: 8px">
                <div>Port: {port}</div>
                <div>Dependencies: {if *has_dependencies { "Yes" } else { "No" }}</div>
            </div>
            <div class="actions">
                <button class="button"><a href="/service/{name}">Manage</a></button>
            </div>
        </div>
        "#}
    }
}
