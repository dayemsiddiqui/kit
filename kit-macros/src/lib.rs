use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, Expr, LitStr, Token};
use std::path::PathBuf;

/// Custom parser for inertia_response! arguments
struct InertiaResponseInput {
    component: LitStr,
    _comma: Token![,],
    props: proc_macro2::TokenStream,
    config: Option<ConfigArg>,
}

struct ConfigArg {
    _comma: Token![,],
    expr: Expr,
}

impl Parse for InertiaResponseInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let component: LitStr = input.parse()?;
        let comma: Token![,] = input.parse()?;

        // Parse the braced props content
        let props_content;
        syn::braced!(props_content in input);
        let props: proc_macro2::TokenStream = props_content.parse()?;

        // Check for optional config argument
        let config = if input.peek(Token![,]) {
            let config_comma: Token![,] = input.parse()?;
            let config_expr: Expr = input.parse()?;
            Some(ConfigArg {
                _comma: config_comma,
                expr: config_expr,
            })
        } else {
            None
        };

        Ok(InertiaResponseInput {
            component,
            _comma: comma,
            props,
            config,
        })
    }
}

/// Create an Inertia response with compile-time component validation
///
/// # Examples
/// ```rust,ignore
/// inertia_response!("Dashboard", { "user": { "name": "John" } })
/// ```
///
/// This macro validates that the component file exists at compile time.
/// If `frontend/src/pages/Dashboard.tsx` doesn't exist, you'll get a compile error.
#[proc_macro]
pub fn inertia_response(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as InertiaResponseInput);

    let component_name = input.component.value();
    let component_lit = &input.component;
    let props = input.props;

    // Validate the component exists at compile time
    if let Err(err) = validate_component_exists(&component_name, component_lit.span()) {
        return err.to_compile_error().into();
    }

    // Generate the appropriate expansion based on whether config is provided
    let expanded = if let Some(config) = input.config {
        let config_expr = config.expr;
        quote! {{
            let props = ::kit::serde_json::json!({#props});
            let url = ::kit::InertiaContext::current_path();
            let response = ::kit::InertiaResponse::new(#component_lit, props, url)
                .with_config(#config_expr);

            if ::kit::InertiaContext::is_inertia_request() {
                Ok(response.to_json_response())
            } else {
                Ok(response.to_html_response())
            }
        }}
    } else {
        quote! {{
            let props = ::kit::serde_json::json!({#props});
            let url = ::kit::InertiaContext::current_path();
            let response = ::kit::InertiaResponse::new(#component_lit, props, url);

            if ::kit::InertiaContext::is_inertia_request() {
                Ok(response.to_json_response())
            } else {
                Ok(response.to_html_response())
            }
        }}
    };

    expanded.into()
}

fn validate_component_exists(component_name: &str, span: Span) -> Result<(), syn::Error> {
    // Get the manifest directory (where Cargo.toml is)
    let manifest_dir = match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(dir) => dir,
        Err(_) => {
            // In environments where CARGO_MANIFEST_DIR isn't set (e.g., some IDEs),
            // skip validation gracefully
            return Ok(());
        }
    };

    let project_root = PathBuf::from(&manifest_dir);

    // Build the expected component path
    // Support nested paths: "Users/Profile" -> frontend/src/pages/Users/Profile.tsx
    let component_path = project_root
        .join("frontend")
        .join("src")
        .join("pages")
        .join(format!("{}.tsx", component_name));

    if !component_path.exists() {
        // Build helpful error message with available components
        let available = list_available_components(&project_root);

        let mut error_msg = format!(
            "Inertia component '{}' not found.\nExpected file: frontend/src/pages/{}.tsx",
            component_name, component_name
        );

        if !available.is_empty() {
            error_msg.push_str("\n\nAvailable components:");
            for comp in &available {
                error_msg.push_str(&format!("\n  - {}", comp));
            }

            // Suggest similar components (fuzzy matching)
            if let Some(suggestion) = find_similar_component(component_name, &available) {
                error_msg.push_str(&format!("\n\nDid you mean '{}'?", suggestion));
            }
        } else {
            error_msg.push_str("\n\nNo components found in frontend/src/pages/");
            error_msg.push_str("\nMake sure your frontend directory structure is set up correctly.");
        }

        return Err(syn::Error::new(span, error_msg));
    }

    Ok(())
}

fn list_available_components(project_root: &PathBuf) -> Vec<String> {
    let pages_dir = project_root
        .join("frontend")
        .join("src")
        .join("pages");

    let mut components = Vec::new();
    collect_components_recursive(&pages_dir, &pages_dir, &mut components);
    components.sort();
    components
}

fn collect_components_recursive(base_dir: &PathBuf, current_dir: &PathBuf, components: &mut Vec<String>) {
    if let Ok(entries) = std::fs::read_dir(current_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();

            if path.is_dir() {
                // Recurse into subdirectories
                collect_components_recursive(base_dir, &path, components);
            } else if path.extension().map(|e| e == "tsx").unwrap_or(false) {
                // Get relative path from pages directory
                if let Ok(relative) = path.strip_prefix(base_dir) {
                    if let Some(stem) = relative.with_extension("").to_str() {
                        // Convert path separators to forward slashes for consistency
                        let component_name = stem.replace(std::path::MAIN_SEPARATOR, "/");
                        components.push(component_name);
                    }
                }
            }
        }
    }
}

fn find_similar_component(target: &str, available: &[String]) -> Option<String> {
    let target_lower = target.to_lowercase();

    // Check for case-insensitive exact match first
    for comp in available {
        if comp.to_lowercase() == target_lower {
            return Some(comp.clone());
        }
    }

    // Find closest match using Levenshtein distance
    let mut best_match: Option<(String, usize)> = None;

    for comp in available {
        let distance = levenshtein_distance(&target_lower, &comp.to_lowercase());
        // Allow up to 2 character differences for short names, more for longer names
        let threshold = std::cmp::max(2, target.len() / 3);
        if distance <= threshold {
            if best_match.is_none() || distance < best_match.as_ref().unwrap().1 {
                best_match = Some((comp.clone(), distance));
            }
        }
    }

    best_match.map(|(name, _)| name)
}

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let len_a = a_chars.len();
    let len_b = b_chars.len();

    if len_a == 0 { return len_b; }
    if len_b == 0 { return len_a; }

    let mut matrix: Vec<Vec<usize>> = vec![vec![0; len_b + 1]; len_a + 1];

    for i in 0..=len_a { matrix[i][0] = i; }
    for j in 0..=len_b { matrix[0][j] = j; }

    for i in 1..=len_a {
        for j in 1..=len_b {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = std::cmp::min(
                std::cmp::min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1),
                matrix[i - 1][j - 1] + cost
            );
        }
    }

    matrix[len_a][len_b]
}
