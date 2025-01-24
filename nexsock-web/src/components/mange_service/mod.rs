use crate::components::mange_service::start::StartService;
use derive_more::Constructor;
use rust_html::{rhtml, Render, Template};

mod start;

const START_SCRIPT: &str = include_str!("start.js");

pub struct ServiceManagementScripts;

impl Render for ServiceManagementScripts {
    fn render(&self) -> Template {
        rhtml! {r#"
            <script>{START_SCRIPT}</script>
        "#}
    }
}

#[derive(Constructor)]
pub struct ServiceManagementForm(i64);

impl Render for ServiceManagementForm {
    fn render(&self) -> Template {
        let start = StartService::new(self.0);

        rhtml! {r#"
            <form class="service-management" onsubmit="handleServiceSubmit(event)">
                <div class="service">
                    <div class="actions">
                        {start}
                    </div>
                </div>
            </form>
        "#}
    }
}
