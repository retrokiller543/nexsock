# Tera Pagination Fix

## Problem
Tera templates cannot call methods on context objects. The current implementation tries to call `branches.displayed_branches()` which fails with:
```
Error: Failed to parse 'git_branches.html' --> 16:49 | 16 | {% for branch in branches.displayed_branches() %}
```

## Solution Options

### Option 1: Pre-compute Pagination in Templates (Recommended)
Modify the template endpoints to do pagination logic server-side and pass simple data structures to templates.

**Changes needed:**

1. **Update `GitBranchesView` component** (`src/components/git_view.rs`):
```rust
#[derive(Debug, Serialize)]
pub struct GitBranchesView {
    pub branches: Vec<String>,           // Already paginated list
    pub service_name: String,
    pub has_more: bool,                  // Simple boolean
    pub remaining_count: usize,          // Simple number
    pub show_all: bool,
}

impl GitBranchesView {
    pub fn new(all_branches: Vec<String>, service_name: String, show_all: bool, limit: usize) -> Self {
        let has_more = !show_all && all_branches.len() > limit;
        let branches = if show_all {
            all_branches.clone()
        } else {
            all_branches.into_iter().take(limit).collect()
        };
        let remaining_count = if show_all { 0 } else { all_branches.len().saturating_sub(limit) };
        
        Self {
            branches,
            service_name,
            has_more,
            remaining_count,
            show_all,
        }
    }
}
```

2. **Update template endpoint** (`src/endpoints/templates.rs`):
```rust
pub async fn git_branches(
    State(ref state): State<AppState>,
    Query(params): Query<GitQuery>,
) -> Result<Html<String>> {
    let service_ref = ServiceRef::from_str(&params.service)?;
    let include_remote = false;
    
    let branches_response = git::list_branches(state, service_ref, include_remote).await?;
    let show_all = params.show_all.unwrap_or(false);
    let limit = params.limit.unwrap_or(10);
    
    let branches_view = GitBranchesView::new(
        branches_response.branches, 
        params.service, 
        show_all, 
        limit
    );
    
    let context = json!({ GitBranchesView::VARIABLE_NAME: branches_view });
    let context = Context::from_serialize(context)?;
    
    let html = TERA.render(GitBranchesView::TEMPLATE_NAME, &context)?;
    Ok(Html(html))
}
```

3. **Update template** (`templates/git_branches.html`):
```html
<!-- Replace method calls with direct field access -->
{% for branch in branches.branches %}
  <!-- content -->
{% endfor %}

{% if branches.has_more %}
  <button>Show {{ branches.remaining_count }} more branches...</button>
{% endif %}
```

4. **Same pattern for GitLogView**:
```rust
#[derive(Debug, Serialize)]
pub struct GitLogView {
    pub commits: Vec<GitCommitInfo>,     // Already paginated
    pub service_name: String,
    pub has_more: bool,
    pub remaining_count: usize,
    pub show_all: bool,
}
```

### Option 2: Custom Tera Functions (More Complex)
Add custom Tera functions for pagination logic.

**Changes needed:**

1. **Create custom Tera functions** (`src/templates.rs`):
```rust
use tera::Function;
use std::collections::HashMap;

fn paginate_list() -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> tera::Result<Value> {
        let list = args.get("list").unwrap().as_array().unwrap();
        let limit = args.get("limit").unwrap().as_u64().unwrap() as usize;
        let show_all = args.get("show_all").unwrap().as_bool().unwrap();
        
        if show_all {
            Ok(Value::Array(list.clone()))
        } else {
            Ok(Value::Array(list.iter().take(limit).cloned().collect()))
        }
    })
}

// Register function with Tera
tera.register_function("paginate", paginate_list());
```

2. **Update templates**:
```html
{% set paginated_branches = paginate(list=branches.branches, limit=10, show_all=branches.show_all) %}
{% for branch in paginated_branches %}
  <!-- content -->
{% endfor %}
```

## Recommended Implementation Steps

1. **Start with Option 1** - it's simpler and more performant
2. **Update GitBranchesView and GitLogView** to pre-compute pagination
3. **Modify template endpoints** to do server-side pagination
4. **Update templates** to use simple field access instead of method calls
5. **Test each component individually** before integrating

## Files to Modify

1. `src/components/git_view.rs` - Simplify component structures
2. `src/endpoints/templates.rs` - Add pagination logic to endpoints  
3. `templates/git_branches.html` - Use field access
4. `templates/git_log.html` - Use field access
5. `templates/git-section.html` - Verify variable names match

## Additional Notes

- Remove the method implementations from GitBranchesView and GitLogView since they won't be used
- Ensure all template variable names match the component field names exactly
- Test with services that have many branches/commits to verify pagination works
- Consider adding loading states for better UX during HTMX requests