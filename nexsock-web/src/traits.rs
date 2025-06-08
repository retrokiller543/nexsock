use crate::error::WebError;
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
    ) -> Result<String, WebError> {
        let mut writer = Vec::new();

        self.render_to(renderer, additional_context, &mut writer)?;

        String::from_utf8(writer).map_err(|utf8_error| {
            WebError::internal(
                "Failed to convert rendered template to UTF-8 string",
                "template_renderer",
                Some(utf8_error),
            )
        })
    }

    #[inline]
    #[tracing::instrument(level = "trace", skip(self, writer))]
    fn render_to(
        &self,
        renderer: &Tera,
        additional_context: Option<TemplateContext>,
        writer: impl Write,
    ) -> Result<(), WebError> {
        let mut context = TemplateContext::new();
        context.insert(Self::VARIABLE_NAME, self);

        if let Some(additional_context) = additional_context {
            context.extend(additional_context);
        }

        render_template_with_context(renderer, Self::TEMPLATE_NAME, &context, writer)
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
    ) -> Result<(), WebError> {
        for item in self {
            item.render_to(renderer, additional_context.clone(), &mut writer)?;
        }

        Ok(())
    }
}

/// Enhanced template rendering with rich error diagnostics
pub fn render_template_with_context(
    renderer: &Tera,
    template_name: &str,
    context: &TemplateContext,
    writer: impl Write,
) -> Result<(), WebError> {
    // Try to get the template source for better error diagnostics
    let template_source = get_template_source(renderer, template_name);

    renderer
        .render_to(template_name, context, writer)
        .map_err(|tera_error| {
            WebError::template_render(
                template_name,
                template_source,
                Some(&context.clone().into_json()),
                tera_error,
            )
        })
}

/// Enhanced template rendering that returns a string
pub fn render_template_to_string(
    renderer: &Tera,
    template_name: &str,
    context: &TemplateContext,
) -> Result<String, WebError> {
    // Try to get the template source for better error diagnostics
    let template_source = get_template_source(renderer, template_name);

    renderer
        .render(template_name, context)
        .map_err(|tera_error| {
            WebError::template_render(
                template_name,
                template_source,
                Some(&context.clone().into_json()),
                tera_error,
            )
        })
}

/// Helper function to safely extract template source from Tera
fn get_template_source(renderer: &Tera, template_name: &str) -> Option<String> {
    // Try to access the template source through Tera's internal structure
    // This is somewhat fragile but provides valuable error context
    renderer
        .get_template(template_name)
        .ok()
        .map(|template| format!("{template:?}"))
}

/// Enhanced context creation with validation
#[allow(dead_code)]
pub fn create_template_context<T: Serialize>(
    variable_name: &str,
    data: &T,
) -> Result<TemplateContext, WebError> {
    let mut context = TemplateContext::new();

    // Validate that the data can be serialized
    if let Err(json_error) = serde_json::to_value(data) {
        return Err(WebError::json_serialize(
            "template context creation",
            std::any::type_name::<T>(),
            json_error,
        ));
    }

    context.insert(variable_name, data);
    Ok(context)
}

/// Enhanced context creation with multiple variables
/// Note: This function is removed due to trait object limitations with Serialize
/// Use individual insert_template_var calls instead
///
/// Safe template variable insertion with validation
#[allow(dead_code)]
pub fn insert_template_var<T: Serialize>(
    context: &mut TemplateContext,
    name: &str,
    value: &T,
) -> Result<(), WebError> {
    // Validate that the value can be serialized
    if let Err(json_error) = serde_json::to_value(value) {
        return Err(WebError::json_serialize(
            format!("template variable '{name}'"),
            std::any::type_name::<T>(),
            json_error,
        ));
    }

    context.insert(name, value);
    Ok(())
}
