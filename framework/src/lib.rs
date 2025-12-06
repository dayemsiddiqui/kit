pub mod http;
pub mod routing;
pub mod server;

pub use http::{Request, Response};
pub use routing::Router;
pub use server::Server;
pub use serde_json::json;

// Re-export for macro usage
#[doc(hidden)]
pub use serde_json;

/// Creates a JSON response directly from a JSON literal.
///
/// # Examples
/// ```ignore
/// json_response!({
///     "users": [{"id": 1, "name": "John"}]
/// })
///
/// // With status code
/// json_response!({"error": "Not found"}).status(404)
/// ```
#[macro_export]
macro_rules! json_response {
    ($($json:tt)+) => {
        $crate::Response::json($crate::serde_json::json!($($json)+))
    };
}
