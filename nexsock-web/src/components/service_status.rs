use crate::components::dependecy_view::DependencyView;
use nexsock_protocol::commands::service_status::ServiceStatus;
use rust_html::{rhtml, Render, Template, TemplateGroup};

pub struct ServiceStatusView(ServiceStatus);

impl ServiceStatusView {
    pub fn new(status: ServiceStatus) -> Self {
        Self(status)
    }
}

impl Render for ServiceStatusView {
    fn render(&self) -> Template {
        let ServiceStatus {
            id,
            name,
            state,
            config_id,
            port,
            repo_url,
            repo_path,
            dependencies,
        } = &self.0;

        let config = if let Some(config) = config_id {
            rhtml! {r#"<div>Config ID: {config}</div>"#}
        } else {
            rhtml! {r#"<div>No Config available for this service</div>"#}
        };

        let deps = DependencyView::from_iter(dependencies.clone())
            .iter()
            .map(|dep| dep.render())
            .collect::<TemplateGroup>();

        rhtml! {r#"
        <div class="service">
            <div>
                <strong>{name}</strong>
                <span style="float: right">ID: {id}</span>
            </div>
            <div style="margin-top: 8px">
                <div>State: {state}</div>
                <div>Port: {port}</div>
                <div>Repository: <a href="{repo_url}">{repo_url}</a></div>
                <div>Path: <a href="file://{repo_path}">{repo_path}</a></div>
                {config}
            </div>
            <div style="margin-top: 16px">
                <h4>Dependencies</h4>
                {deps}
            </div>
        </div>
        "#}
    }
}
