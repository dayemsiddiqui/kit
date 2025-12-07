pub mod config;
pub mod http;
pub mod inertia;
pub mod routing;
pub mod server;

pub use config::{
    env, env_optional, env_required, AppConfig, Config, Environment, ServerConfig,
};
pub use http::{json, text, HttpResponse, Redirect, Request, Response, ResponseExt};
pub use inertia::{InertiaConfig, InertiaContext, InertiaResponse};
pub use routing::{route, RouteBuilder, Router};
pub use server::Server;

// Re-export for macro usage
#[doc(hidden)]
pub use serde_json;

// Re-export serde for InertiaProps derive macro
pub use serde;

// Re-export the proc-macros for compile-time component validation and type safety
pub use kit_macros::inertia_response;
pub use kit_macros::InertiaProps;
pub use kit_macros::redirect;

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
