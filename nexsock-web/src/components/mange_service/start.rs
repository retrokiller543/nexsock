use derive_more::{Constructor, From, Into, IntoIterator};
use rust_html::{rhtml, Render, Template};

#[derive(Debug, From, Into, Constructor)]
pub struct StartService(i64);

impl StartService {
    pub fn from_iter(iter: impl IntoIterator<Item = i64>) -> Vec<StartService> {
        iter.into_iter().map(Self::new).collect()
    }
}

impl Render for StartService {
    fn render(&self) -> Template {
        rhtml! {r#"
                <div class="service">
                    <div class="status">Service: {self.0}</div>
                    <div class="actions">
                        <button
                            type="submit"
                            onclick="startService('{self.0}')"
                            class="button"
                            data-service-id="{self.0}"
                        >
                            Start {self.0}
                        </button>
                    </div>
                </div>
            "#}
    }
}
