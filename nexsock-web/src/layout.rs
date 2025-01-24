use crate::components::mange_service::ServiceManagementScripts;
use rust_html::{rhtml, Render, Template, Unescaped};

fn page_style() -> Template {
    let style = r#"<style>
        body {
            font-family: system-ui, -apple-system, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
        }
        .status {
            padding: 15px;
            border-radius: 4px;
            background: #f0f0f0;
            margin: 20px 0;
        }
        .service {
            border: 1px solid #ddd;
            padding: 10px;
            margin: 10px 0;
            border-radius: 4px;
        }
        .actions {
            margin-top: 10px;
        }
        .button {
            background: #0070f3;
            color: white;
            border: none;
            padding: 8px 16px;
            border-radius: 4px;
            cursor: pointer;
        }
    </style>"#;
    Unescaped(style.to_string()).render()
}

pub struct Layout<T: Render>(T);

impl<T: Render> Layout<T> {
    pub fn new(renderable: T) -> Self {
        Self(renderable)
    }
}

impl<T: Render> Render for Layout<T> {
    fn render(&self) -> Template {
        let style = page_style();
        let scripts = ServiceManagementScripts;
        rhtml! {r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Service Manager</title>
            {style}
        </head>
        <body>
            <nav>
                <a href="/">Services</a>
            </nav>
            {self.0}
            {scripts}
        </body>
        </html>
        "#}
    }
}
