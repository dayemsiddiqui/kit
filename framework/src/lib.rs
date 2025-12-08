pub mod config;
pub mod container;
pub mod http;
pub mod inertia;
pub mod middleware;
pub mod routing;
pub mod server;

pub use config::{
    env, env_optional, env_required, AppConfig, Config, Environment, ServerConfig,
};
pub use container::{App, Container};
pub use http::{json, text, HttpResponse, Redirect, Request, Response, ResponseExt};
pub use inertia::{InertiaConfig, InertiaContext, InertiaResponse};
pub use middleware::{Middleware, MiddlewareFuture, MiddlewareRegistry, Next};
pub use routing::{delete, get, post, put, route, GroupBuilder, GroupRouter, RouteBuilder, RouteDefBuilder, Router};
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

// Re-export the proc-macros for compile-time component validation and type safety
pub use kit_macros::inertia_response;
pub use kit_macros::injectable;
pub use kit_macros::InertiaProps;
pub use kit_macros::redirect;
pub use kit_macros::service;

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

/// Testing utilities for the application container
///
/// Provides `TestContainer` for setting up isolated test environments with
/// fake implementations.
pub mod testing {
    pub use crate::container::testing::{TestContainer, TestContainerGuard};
}
