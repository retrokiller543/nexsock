use miette::{LabeledSpan, SourceSpan};
use serde_json::Value;
use std::collections::HashSet;

/// Helper functions for calculating source spans
pub fn calculate_json_error_span(
    json_content: &str,
    error: &serde_json::Error,
) -> Option<SourceSpan> {
    let line = error.line();
    let column = error.column();

    if line > 0 && column > 0 {
        let lines: Vec<&str> = json_content.lines().collect();
        let mut offset = 0;

        for i in 0..(line - 1).min(lines.len()) {
            offset += lines[i].len() + 1;
        }

        offset += column.saturating_sub(1);

        let span_length = estimate_json_token_length(&json_content[offset..]);

        return Some(SourceSpan::new(offset.into(), span_length));
    }
    None
}

pub fn calculate_template_error_span_with_suggestions(
    template_content: &str,
    error: &tera::Error,
    context_json: &str,
) -> (Option<SourceSpan>, Vec<LabeledSpan>) {
    let error_msg = get_deepest_error_message(error);

    if let Some(missing_variable) = extract_variable_from_tera_error(&error_msg) {
        let available_vars = extract_context_variables(context_json);
        let primary_span = find_missing_variable_in_template(template_content, &missing_variable);
        let secondary_spans = find_similar_variables_in_template(
            template_content,
            &missing_variable,
            &available_vars,
        );

        (primary_span, secondary_spans)
    } else {
        (
            find_template_syntax_error_span(template_content, &error_msg),
            Vec::new(),
        )
    }
}

/// Get the deepest/most specific error message from a chain of errors
fn get_deepest_error_message(error: &tera::Error) -> String {
    let mut current: &dyn std::error::Error = error;
    let mut deepest_msg = error.to_string();

    while let Some(source) = current.source() {
        deepest_msg = source.to_string();
        current = source;
    }

    deepest_msg
}

pub fn calculate_form_field_span(
    form_content: &str,
    field_name: &str,
    _field_value: &str,
) -> Option<SourceSpan> {
    if let Some(field_start) = form_content.find(field_name) {
        return Some(SourceSpan::new(field_start.into(), field_name.len()));
    }
    None
}

pub fn calculate_query_param_span(query_string: &str, param_name: &str) -> Option<SourceSpan> {
    if let Some(param_start) = query_string.find(param_name) {
        return Some(SourceSpan::new(param_start.into(), param_name.len()));
    }
    None
}

/// Extract context variables from JSON string, flattening nested objects with dot notation
fn extract_context_variables(context_json: &str) -> HashSet<String> {
    let mut variables = HashSet::new();

    if let Ok(value) = serde_json::from_str::<Value>(context_json) {
        extract_keys_recursive(&value, String::new(), &mut variables);
    }

    variables
}

/// Recursively extract all keys from JSON value, using dot notation for nested objects
fn extract_keys_recursive(value: &Value, prefix: String, keys: &mut HashSet<String>) {
    match value {
        Value::Object(map) => {
            for (key, val) in map {
                let full_key = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", prefix, key)
                };

                keys.insert(full_key.clone());
                extract_keys_recursive(val, full_key, keys);
            }
        }
        Value::Array(arr) => {
            for (i, val) in arr.iter().enumerate() {
                let full_key = format!("{}[{}]", prefix, i);
                keys.insert(full_key.clone());
                extract_keys_recursive(val, full_key, keys);
            }
        }
        _ => {}
    }
}

/// Find the exact location of a missing variable in template
fn find_missing_variable_in_template(
    template_content: &str,
    variable_name: &str,
) -> Option<SourceSpan> {
    let variable_patterns = [
        format!("{{{{ {} }}}}", variable_name),
        format!(
            "{{{{ {}.* }}}}",
            variable_name.split('.').next().unwrap_or(variable_name)
        ),
        format!("{{{{ {} | ", variable_name),
        format!("{{{{ {} }}", variable_name),
    ];

    for pattern in &variable_patterns {
        if let Some(pos) = find_pattern_in_template(template_content, pattern) {
            let var_start = pos + 3;
            return Some(SourceSpan::new(var_start.into(), variable_name.len()));
        }
    }

    if let Some(pos) = template_content.find(variable_name) {
        return Some(SourceSpan::new(pos.into(), variable_name.len()));
    }

    None
}

/// Find pattern in template content accounting for whitespace variations
fn find_pattern_in_template(template_content: &str, pattern: &str) -> Option<usize> {
    template_content.find(pattern)
}

/// Find all potential variable errors in template and suggest corrections
fn find_similar_variables_in_template(
    template_content: &str,
    missing_variable: &str,
    available_vars: &HashSet<String>,
) -> Vec<LabeledSpan> {
    let mut spans = Vec::new();

    // Find all variables used in the template
    let template_vars = extract_all_template_variables(template_content);

    // For each template variable, check if it's missing from context and if we have similar suggestions
    for template_var in template_vars {
        // Skip the primary error variable (already handled)
        if template_var == missing_variable {
            continue;
        }

        // Check if this variable exists in the available context
        if !available_vars.contains(&template_var) {
            // This variable is also missing, find suggestions for it
            let similar_vars: Vec<&String> = available_vars
                .iter()
                .filter(|var| is_variable_similar(&template_var, var))
                .take(1) // Only suggest the best match
                .collect();

            for similar_var in similar_vars {
                if let Some(span) =
                    find_missing_variable_in_template(template_content, &template_var)
                {
                    spans.push(LabeledSpan::new(
                        Some(format!("Did you mean '{}'?", similar_var)),
                        span.offset(),
                        span.len(),
                    ));
                }
            }
        }
    }

    spans
}

/// Check if two variable names are similar (for suggestions)
fn is_variable_similar(target: &str, candidate: &str) -> bool {
    let target_lower = target.to_lowercase();
    let candidate_lower = candidate.to_lowercase();

    if target_lower == candidate_lower {
        return false;
    }

    // Don't suggest if the candidate is much shorter (likely a false positive)
    if candidate_lower.len() < target_lower.len().saturating_sub(3) {
        return false;
    }

    // Check for exact typos with small edit distance
    let edit_distance = strsim::levenshtein(&target_lower, &candidate_lower);
    if edit_distance <= 2 && edit_distance > 0 {
        // Make sure they're reasonably similar in length for typo detection
        let length_diff = (target_lower.len() as i32 - candidate_lower.len() as i32).abs();
        if length_diff <= 2 {
            return true;
        }
    }

    // Check for substring matches only if they're similar length
    if target_lower.len() >= 3 && candidate_lower.len() >= 3 {
        let length_ratio = target_lower.len() as f32 / candidate_lower.len() as f32;
        if length_ratio >= 0.7 && length_ratio <= 1.3 {
            if candidate_lower
                .contains(&target_lower[..target_lower.len().min(candidate_lower.len())])
                || target_lower
                    .contains(&candidate_lower[..candidate_lower.len().min(target_lower.len())])
            {
                return true;
            }
        }
    }

    false
}

fn extract_variable_from_tera_error(error_msg: &str) -> Option<String> {
    if let Some(captures) = extract_with_pattern(error_msg, "variable") {
        return Some(captures);
    }

    if let Some(captures) = extract_with_pattern(error_msg, "filter") {
        return Some(format!("| {}", captures));
    }

    if let Some(captures) = extract_with_pattern(error_msg, "function") {
        return Some(captures);
    }

    if let Some(captures) = extract_with_pattern(error_msg, "block") {
        return Some(format!("{{% block {} %}}", captures));
    }

    if let Some(captures) = extract_with_pattern(error_msg, "test") {
        return Some(format!("is {}", captures));
    }

    None
}

/// Helper function to extract patterns using simple string operations
fn extract_with_pattern(text: &str, pattern_type: &str) -> Option<String> {
    let prefixes = match pattern_type {
        "variable" => vec!["Variable `", "variable `"],
        "filter" => vec!["Filter `", "filter `"],
        "function" => vec!["Function `", "function `"],
        "block" => vec!["Block `", "block `"],
        "test" => vec!["Test `", "test `"],
        _ => return None,
    };

    for prefix in prefixes {
        if let Some(start) = text.find(prefix) {
            let after_start = &text[start + prefix.len()..];
            if let Some(end) = after_start.find('`') {
                return Some(after_start[..end].to_string());
            }
        }
    }

    None
}

fn find_variable_span_with_suggestions(
    template_content: &str,
    variable_name: &str,
    error_msg: &str,
) -> Option<SourceSpan> {
    if let Some(span) = find_variable_span_in_template(template_content, variable_name) {
        return Some(span);
    }

    let potential_variables = extract_all_template_variables(template_content);

    for potential_var in &potential_variables {
        if is_similar_variable_name(variable_name, potential_var) {
            if let Some(span) = find_variable_span_in_template(template_content, potential_var) {
                return Some(span);
            }
        }
    }

    if error_msg.contains("not found") {
        if let Some(span) = find_scope_definition_span(template_content, variable_name) {
            return Some(span);
        }
    }

    None
}

fn extract_all_template_variables(template_content: &str) -> Vec<String> {
    let mut variables = Vec::new();

    // Use find() method which is UTF-8 safe
    let mut search_from = 0;
    while let Some(start) = template_content[search_from..].find("{{") {
        let absolute_start = search_from + start;

        if let Some(end) = template_content[absolute_start..].find("}}") {
            let content_start = absolute_start + 2;
            let content_end = absolute_start + end;

            if content_end > content_start {
                let content = template_content[content_start..content_end].trim();

                // Extract the variable name (before any filters or operations)
                if let Some(var_name) = content.split_whitespace().next() {
                    // Clean up the variable name (remove filters but keep full paths)
                    let clean_var = var_name.split('|').next().unwrap_or(var_name).trim();
                    if !clean_var.is_empty() && !variables.contains(&clean_var.to_string()) {
                        variables.push(clean_var.to_string());
                    }
                }
            }

            search_from = absolute_start + end + 2;
        } else {
            break;
        }
    }

    variables
}

fn is_similar_variable_name(target: &str, candidate: &str) -> bool {
    let target_lower = target.to_lowercase();
    let candidate_lower = candidate.to_lowercase();

    if target_lower == candidate_lower {
        return true;
    }

    if target_lower.contains(&candidate_lower) || candidate_lower.contains(&target_lower) {
        return true;
    }

    let target_snake = target_lower.replace('_', "");
    let candidate_snake = candidate_lower.replace('_', "");
    if target_snake == candidate_snake {
        return true;
    }

    false
}

fn find_scope_definition_span(template_content: &str, variable_name: &str) -> Option<SourceSpan> {
    let for_pattern = format!("{{% for {} in", variable_name);
    if let Some(pos) = template_content.find(&for_pattern) {
        let var_start = pos + 9;
        return Some(SourceSpan::new(var_start.into(), variable_name.len()));
    }

    let set_pattern = format!("{{% set {} =", variable_name);
    if let Some(pos) = template_content.find(&set_pattern) {
        let var_start = pos + 8;
        return Some(SourceSpan::new(var_start.into(), variable_name.len()));
    }

    None
}

fn find_variable_span_in_template(
    template_content: &str,
    variable_name: &str,
) -> Option<SourceSpan> {
    for pattern in &[
        format!("{{ {} }}", variable_name),
        format!(
            "{{ {}.",
            variable_name.split('.').next().unwrap_or(variable_name)
        ),
    ] {
        if let Some(pos) = template_content.find(pattern) {
            let variable_start = pos + 3;
            return Some(SourceSpan::new(variable_start.into(), variable_name.len()));
        }
    }

    find_template_variable_by_regex(template_content, variable_name)
}

fn find_template_variable_by_regex(
    template_content: &str,
    variable_name: &str,
) -> Option<SourceSpan> {
    let lines: Vec<&str> = template_content.lines().collect();
    let mut current_offset = 0;

    for line in lines {
        if line.contains(&format!("{{ {}", variable_name))
            || line.contains(&format!("{{% .* {} %}}", variable_name))
        {
            if let Some(pos_in_line) = line.find(variable_name) {
                return Some(SourceSpan::new(
                    (current_offset + pos_in_line).into(),
                    variable_name.len(),
                ));
            }
        }
        current_offset += line.len() + 1;
    }

    if let Some(pos) = template_content.find(variable_name) {
        return Some(SourceSpan::new(pos.into(), variable_name.len()));
    }

    None
}

fn find_template_syntax_error_span(template_content: &str, error_msg: &str) -> Option<SourceSpan> {
    if error_msg.contains("unexpected") || error_msg.contains("syntax") {
        let problematic_patterns = ["{%", "{{", "%}", "}}", "endif", "endfor", "else", "elif"];

        for pattern in &problematic_patterns {
            if error_msg.contains(pattern) {
                if let Some(pos) = template_content.find(pattern) {
                    return Some(SourceSpan::new(pos.into(), pattern.len()));
                }
            }
        }
    }

    None
}

fn estimate_json_token_length(remaining_content: &str) -> usize {
    let chars: Vec<char> = remaining_content.chars().collect();

    if chars.is_empty() {
        return 1;
    }

    match chars[0] {
        '"' => {
            let mut i = 1;
            let mut escaped = false;
            while i < chars.len() {
                if !escaped && chars[i] == '"' {
                    return i + 1;
                }
                escaped = chars[i] == '\\' && !escaped;
                i += 1;
            }
            chars.len()
        }
        '{' | '[' => 1,
        '}' | ']' => 1,
        ',' | ':' => 1,
        _ => chars
            .iter()
            .take_while(|&&c| c.is_alphanumeric() || c == '.' || c == '-' || c == '+')
            .count()
            .max(1),
    }
}
