use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::path::PathBuf;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, DeriveInput, Expr, LitStr, Token};

use crate::utils::levenshtein_distance;

/// Props can be either a typed struct expression or JSON-like syntax
pub enum PropsKind {
    /// Typed struct: `HomeProps { title: "Welcome".into(), user }`
    Typed(Expr),
    /// JSON-like syntax: `{ "title": "Welcome" }`
    Json(proc_macro2::TokenStream),
}

/// Custom parser for inertia_response! arguments
pub struct InertiaResponseInput {
    pub component: LitStr,
    pub _comma: Token![,],
    pub props: PropsKind,
    pub config: Option<ConfigArg>,
}

pub struct ConfigArg {
    pub _comma: Token![,],
    pub expr: Expr,
}

impl Parse for InertiaResponseInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let component: LitStr = input.parse()?;
        let comma: Token![,] = input.parse()?;

        // Determine if this is a typed struct or JSON syntax
        // Typed struct: identifier followed by { }
        // JSON syntax: directly { }
        let props = if input.peek(syn::Ident) {
            // This is a typed struct expression: `HomeProps { ... }`
            let expr: Expr = input.parse()?;
            PropsKind::Typed(expr)
        } else {
            // This is JSON-like syntax: `{ "key": value }`
            let props_content;
            syn::braced!(props_content in input);
            let props_tokens: proc_macro2::TokenStream = props_content.parse()?;
            PropsKind::Json(props_tokens)
        };

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

/// Implementation for the InertiaProps derive macro
pub fn derive_inertia_props_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Extract field information for generating Serialize impl
    let fields = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => &fields.named,
            _ => {
                return syn::Error::new_spanned(
                    &input,
                    "InertiaProps only supports structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(&input, "InertiaProps can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    let field_count = fields.len();
    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let field_name_strings: Vec<_> = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap().to_string())
        .collect();

    let expanded = quote! {
        impl #impl_generics ::kit::serde::Serialize for #name #ty_generics #where_clause {
            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::kit::serde::Serializer,
            {
                use ::kit::serde::ser::SerializeStruct;
                let mut state = serializer.serialize_struct(stringify!(#name), #field_count)?;
                #(
                    state.serialize_field(#field_name_strings, &self.#field_names)?;
                )*
                state.end()
            }
        }
    };

    expanded.into()
}

/// Implementation for the inertia_response! macro
pub fn inertia_response_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as InertiaResponseInput);

    let component_name = input.component.value();
    let component_lit = &input.component;

    // Validate the component exists at compile time
    if let Err(err) = validate_component_exists(&component_name, component_lit.span()) {
        return err.to_compile_error().into();
    }

    // Generate props conversion based on props kind
    let props_expr = match &input.props {
        PropsKind::Typed(expr) => {
            // Typed struct: serialize using serde_json::to_value
            quote! {
                ::kit::serde_json::to_value(&#expr)
                    .expect("Failed to serialize InertiaProps")
            }
        }
        PropsKind::Json(tokens) => {
            // JSON-like syntax: use serde_json::json! macro
            quote! {
                ::kit::serde_json::json!({#tokens})
            }
        }
    };

    // Generate the appropriate expansion based on whether config is provided
    let expanded = if let Some(config) = input.config {
        let config_expr = config.expr;
        quote! {{
            let props = #props_expr;
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
            let props = #props_expr;
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
    let pages_dir = project_root.join("frontend").join("src").join("pages");

    let mut components = Vec::new();
    collect_components_recursive(&pages_dir, &pages_dir, &mut components);
    components.sort();
    components
}

fn collect_components_recursive(
    base_dir: &PathBuf,
    current_dir: &PathBuf,
    components: &mut Vec<String>,
) {
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
