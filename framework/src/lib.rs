pub mod http;
pub mod inertia;
pub mod routing;
pub mod server;

pub use http::{json, text, HttpResponse, Request, Response, ResponseExt};
pub use inertia::{InertiaConfig, InertiaContext, InertiaResponse};
pub use routing::Router;
pub use server::Server;

// Re-export for macro usage
#[doc(hidden)]
pub use serde_json;

#[macro_export]
macro_rules! json_response {
    ($($json:tt)+) => {
        Ok($crate::HttpResponse::json($crate::serde_json::json!($($json)+)))
    };
}

#[macro_export]
macro_rules! text_response {
    ($text:expr) => {
        Ok($crate::HttpResponse::text($text))
    };
}

/// Create an Inertia response - automatically detects request type from context
///
/// # Examples
/// ```rust
/// inertia_response!("Dashboard", { "user": { "name": "John" } })
/// ```
#[macro_export]
macro_rules! inertia_response {
    ($component:expr, $props:tt) => {{
        let props = $crate::serde_json::json!($props);
        let url = $crate::InertiaContext::current_path();
        let response = $crate::InertiaResponse::new($component, props, url);

        if $crate::InertiaContext::is_inertia_request() {
            Ok(response.to_json_response())
        } else {
            Ok(response.to_html_response())
        }
    }};

    ($component:expr, $props:tt, $config:expr) => {{
        let props = $crate::serde_json::json!($props);
        let url = $crate::InertiaContext::current_path();
        let response = $crate::InertiaResponse::new($component, props, url)
            .with_config($config);

        if $crate::InertiaContext::is_inertia_request() {
            Ok(response.to_json_response())
        } else {
            Ok(response.to_html_response())
        }
    }};
}
