pub mod config;
pub mod container;
pub mod database;
pub mod error;
pub mod http;
pub mod inertia;
pub mod middleware;
pub mod routing;
pub mod server;

pub use config::{env, env_optional, env_required, AppConfig, Config, Environment, ServerConfig};
pub use container::{App, Container};
pub use database::{
    AutoRouteBinding, Database, DatabaseConfig, DatabaseType, DbConnection, Model, ModelMut,
    RouteBinding, DB,
};
pub use error::{AppError, FrameworkError, HttpError, ValidationErrors};
pub use http::{
    json, text, FormRequest, FromParam, FromRequest, HttpResponse, Redirect, Request, Response,
    ResponseExt,
};
pub use inertia::{InertiaConfig, InertiaContext, InertiaResponse};
pub use middleware::{
    register_global_middleware, Middleware, MiddlewareFuture, MiddlewareRegistry, Next,
};
pub use routing::{
    route, validate_route_path,
    // Internal functions used by macros (hidden from docs)
    __delete_impl, __fallback_impl, __get_impl, __post_impl, __put_impl,
    FallbackDefBuilder, GroupBuilder, GroupDef, GroupRoute, GroupRouter, RouteBuilder,
    RouteDefBuilder, Router,
};
pub use server::Server;

// Re-export async_trait for middleware implementations
pub use async_trait::async_trait;

// Re-export inventory for #[service(ConcreteType)] macro
#[doc(hidden)]
pub use inventory;

// Re-export for macro usage
#[doc(hidden)]
pub use serde_json;

// Re-export serde for InertiaProps derive macro
pub use serde;

// Re-export validator for FormRequest validation
pub use validator;
pub use validator::Validate;

// Re-export the proc-macros for compile-time component validation and type safety
pub use kit_macros::domain_error;
pub use kit_macros::handler;
pub use kit_macros::inertia_response;
pub use kit_macros::injectable;
pub use kit_macros::redirect;
pub use kit_macros::request;
pub use kit_macros::service;
pub use kit_macros::FormRequest as FormRequestDerive;
pub use kit_macros::InertiaProps;

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

/// Register global middleware that runs on every request
///
/// Global middleware is registered in `bootstrap.rs` and runs in registration order,
/// before any route-specific middleware.
///
/// # Example
///
/// ```rust,ignore
/// // In bootstrap.rs
/// use kit::global_middleware;
/// use crate::middleware;
///
/// pub fn register() {
///     global_middleware!(middleware::LoggingMiddleware);
///     global_middleware!(middleware::CorsMiddleware);
/// }
/// ```
#[macro_export]
macro_rules! global_middleware {
    ($middleware:expr) => {
        $crate::register_global_middleware($middleware)
    };
}

/// Testing utilities for the application container
///
/// Provides `TestContainer` for setting up isolated test environments with
/// fake implementations.
pub mod testing {
    pub use crate::container::testing::{TestContainer, TestContainerGuard};
}
