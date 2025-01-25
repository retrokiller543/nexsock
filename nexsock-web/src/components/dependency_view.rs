use crate::traits::RenderTemplate;
use derive_more::{AsMut, AsRef, Constructor, Deref, DerefMut, From, Into};
use nexsock_protocol::commands::dependency_info::DependencyInfo;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Constructor, AsRef, AsMut, Deref, DerefMut, From, Into)]
pub struct DependencyView(DependencyInfo);

#[allow(dead_code)]
impl DependencyView {
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
