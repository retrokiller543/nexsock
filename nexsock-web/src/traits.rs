use anyhow::Context;
use serde::Serialize;
use std::io::Write;
use tera::{Context as TemplateContext, Tera};

pub trait RenderTemplate: Serialize {
    const TEMPLATE_NAME: &'static str;
    const VARIABLE_NAME: &'static str;

    #[inline]
    #[tracing::instrument(level = "trace", skip(self))]
    fn render(
        &self,
        renderer: &Tera,
        additional_context: Option<TemplateContext>,
    ) -> anyhow::Result<String> {
        let mut writer = Vec::new();

        self.render_to(renderer, additional_context, &mut writer)?;

        String::from_utf8(writer).context("failed to convert rendered template to UTF-8 string")
    }

    #[inline]
    #[tracing::instrument(level = "trace", skip(self, writer))]
    fn render_to(
        &self,
        renderer: &Tera,
        additional_context: Option<TemplateContext>,
        writer: impl Write,
    ) -> anyhow::Result<()> {
        let mut context = TemplateContext::new();
        context.insert(Self::VARIABLE_NAME, self);

        if let Some(additional_context) = additional_context {
            context.extend(additional_context);
        }

        renderer
            .render_to(Self::TEMPLATE_NAME, &context, writer)
            .context("failed to render template")
    }
}

impl<T: RenderTemplate> RenderTemplate for Vec<T> {
    const TEMPLATE_NAME: &'static str = T::TEMPLATE_NAME;
    const VARIABLE_NAME: &'static str = T::VARIABLE_NAME;

    #[inline]
    #[tracing::instrument(level = "trace", skip(self, writer))]
    fn render_to(
        &self,
        renderer: &Tera,
        additional_context: Option<TemplateContext>,
        mut writer: impl Write,
    ) -> anyhow::Result<()> {
        for item in self {
            item.render_to(renderer, additional_context.clone(), &mut writer)?;
        }

        Ok(())
    }
}
