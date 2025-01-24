use crate::traits::RenderTemplate;
use nexsock_protocol::commands::dependency_info::DependencyInfo;
use rust_html::{rhtml, Render, Template};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct DependencyView(DependencyInfo);

impl DependencyView {
    pub fn new(dep: DependencyInfo) -> Self {
        Self(dep)
    }

    pub fn from_iter(iter: impl IntoIterator<Item = DependencyInfo>) -> Vec<Self> {
        let mut dependencies = Vec::new();

        for dependency in iter {
            dependencies.push(Self::new(dependency));
        }

        dependencies
    }
}

impl RenderTemplate for DependencyView {
    const TEMPLATE_NAME: &'static str = "dependency.html";
    const VARIABLE_NAME: &'static str = "dependency";
}

impl Render for DependencyView {
    fn render(&self) -> Template {
        let DependencyInfo {
            id,
            name,
            tunnel_enabled,
        } = &self.0;

        rhtml! {r#"
        <div class="service">
            <div>
                <strong>{name}</strong>
                <span style="float: right">ID: {id}</span>
            </div>
            <div style="margin-top: 8px">
                Tunnel: {if *tunnel_enabled { "Enabled" } else { "Disabled" }}
            </div>
        </div>
        "#}
    }
}
