//! Route definition macros and helpers for Laravel-like routing syntax
//!
//! This module provides a clean, declarative way to define routes:
//!
//! ```rust,ignore
//! use kit::{routes, get, post, put, delete, group};
//!
//! routes! {
//!     get!("/", controllers::home::index).name("home"),
//!     get!("/users", controllers::user::index).name("users.index"),
//!     post!("/users", controllers::user::store).name("users.store"),
//!     get!("/protected", controllers::home::index).middleware(AuthMiddleware),
//!
//!     // Route groups with prefix and middleware
//!     group!("/api", {
//!         get!("/users", controllers::api::user::index).name("api.users.index"),
//!         post!("/users", controllers::api::user::store).name("api.users.store"),
//!     }).middleware(AuthMiddleware),
//! }
//! ```

use crate::http::{Request, Response};

/// Const function to validate route paths start with '/'
///
/// This provides compile-time validation that all route paths begin with '/'.
/// If a path doesn't start with '/', compilation will fail with a clear error.
///
/// # Panics
///
/// Panics at compile time if the path is empty or doesn't start with '/'.
pub const fn validate_route_path(path: &'static str) -> &'static str {
    let bytes = path.as_bytes();
    if bytes.is_empty() || bytes[0] != b'/' {
        panic!("Route path must start with '/'")
    }
    path
}
use crate::middleware::{into_boxed, BoxedMiddleware, Middleware};
use crate::routing::router::{register_route_name, BoxedHandler, Router};
use std::future::Future;
use std::sync::Arc;

/// HTTP method for route definitions
#[derive(Clone, Copy)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

/// Builder for route definitions that supports `.name()` and `.middleware()` chaining
pub struct RouteDefBuilder<H> {
    method: HttpMethod,
    path: &'static str,
    handler: H,
    name: Option<&'static str>,
    middlewares: Vec<BoxedMiddleware>,
}

impl<H, Fut> RouteDefBuilder<H>
where
    H: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    /// Create a new route definition builder
    pub fn new(method: HttpMethod, path: &'static str, handler: H) -> Self {
        Self {
            method,
            path,
            handler,
            name: None,
            middlewares: Vec::new(),
        }
    }

    /// Name this route for URL generation
    pub fn name(mut self, name: &'static str) -> Self {
        self.name = Some(name);
        self
    }

    /// Add middleware to this route
    pub fn middleware<M: Middleware + 'static>(mut self, middleware: M) -> Self {
        self.middlewares.push(into_boxed(middleware));
        self
    }

    /// Register this route definition with a router
    pub fn register(self, router: Router) -> Router {
        // First, register the route based on method
        let builder = match self.method {
            HttpMethod::Get => router.get(self.path, self.handler),
            HttpMethod::Post => router.post(self.path, self.handler),
            HttpMethod::Put => router.put(self.path, self.handler),
            HttpMethod::Delete => router.delete(self.path, self.handler),
        };

        // Apply any middleware
        let builder = self
            .middlewares
            .into_iter()
            .fold(builder, |b, m| b.middleware_boxed(m));

        // Apply name if present, otherwise convert to Router
        if let Some(name) = self.name {
            builder.name(name)
        } else {
            builder.into()
        }
    }
}

/// Create a GET route definition with compile-time path validation
///
/// # Example
/// ```rust,ignore
/// get!("/users", controllers::user::index).name("users.index")
/// ```
///
/// # Compile Error
///
/// Fails to compile if path doesn't start with '/'.
#[macro_export]
macro_rules! get {
    ($path:expr, $handler:expr) => {{
        const _: &str = $crate::validate_route_path($path);
        $crate::__get_impl($path, $handler)
    }};
}

/// Internal implementation for GET routes (used by the get! macro)
#[doc(hidden)]
pub fn __get_impl<H, Fut>(path: &'static str, handler: H) -> RouteDefBuilder<H>
where
    H: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    RouteDefBuilder::new(HttpMethod::Get, path, handler)
}

/// Create a POST route definition with compile-time path validation
///
/// # Example
/// ```rust,ignore
/// post!("/users", controllers::user::store).name("users.store")
/// ```
///
/// # Compile Error
///
/// Fails to compile if path doesn't start with '/'.
#[macro_export]
macro_rules! post {
    ($path:expr, $handler:expr) => {{
        const _: &str = $crate::validate_route_path($path);
        $crate::__post_impl($path, $handler)
    }};
}

/// Internal implementation for POST routes (used by the post! macro)
#[doc(hidden)]
pub fn __post_impl<H, Fut>(path: &'static str, handler: H) -> RouteDefBuilder<H>
where
    H: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    RouteDefBuilder::new(HttpMethod::Post, path, handler)
}

/// Create a PUT route definition with compile-time path validation
///
/// # Example
/// ```rust,ignore
/// put!("/users/{id}", controllers::user::update).name("users.update")
/// ```
///
/// # Compile Error
///
/// Fails to compile if path doesn't start with '/'.
#[macro_export]
macro_rules! put {
    ($path:expr, $handler:expr) => {{
        const _: &str = $crate::validate_route_path($path);
        $crate::__put_impl($path, $handler)
    }};
}

/// Internal implementation for PUT routes (used by the put! macro)
#[doc(hidden)]
pub fn __put_impl<H, Fut>(path: &'static str, handler: H) -> RouteDefBuilder<H>
where
    H: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    RouteDefBuilder::new(HttpMethod::Put, path, handler)
}

/// Create a DELETE route definition with compile-time path validation
///
/// # Example
/// ```rust,ignore
/// delete!("/users/{id}", controllers::user::destroy).name("users.destroy")
/// ```
///
/// # Compile Error
///
/// Fails to compile if path doesn't start with '/'.
#[macro_export]
macro_rules! delete {
    ($path:expr, $handler:expr) => {{
        const _: &str = $crate::validate_route_path($path);
        $crate::__delete_impl($path, $handler)
    }};
}

/// Internal implementation for DELETE routes (used by the delete! macro)
#[doc(hidden)]
pub fn __delete_impl<H, Fut>(path: &'static str, handler: H) -> RouteDefBuilder<H>
where
    H: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    RouteDefBuilder::new(HttpMethod::Delete, path, handler)
}

// ============================================================================
// Route Grouping Support
// ============================================================================

/// A route stored within a group (type-erased handler)
pub struct GroupRoute {
    method: HttpMethod,
    path: &'static str,
    handler: Arc<BoxedHandler>,
    name: Option<&'static str>,
    middlewares: Vec<BoxedMiddleware>,
}

/// Group definition that collects routes and applies prefix/middleware
///
/// # Example
///
/// ```rust,ignore
/// routes! {
///     group!("/api", {
///         get!("/users", controllers::user::index).name("api.users"),
///         post!("/users", controllers::user::store),
///     }).middleware(AuthMiddleware),
/// }
/// ```
pub struct GroupDef {
    prefix: &'static str,
    routes: Vec<GroupRoute>,
    group_middlewares: Vec<BoxedMiddleware>,
}

impl GroupDef {
    /// Create a new route group with the given prefix (internal use)
    ///
    /// Use the `group!` macro instead for compile-time validation.
    #[doc(hidden)]
    pub fn __new_unchecked(prefix: &'static str) -> Self {
        Self {
            prefix,
            routes: Vec::new(),
            group_middlewares: Vec::new(),
        }
    }

    /// Add a route to this group
    pub fn route<H, Fut>(mut self, route: RouteDefBuilder<H>) -> Self
    where
        H: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        self.routes.push(route.into_group_route());
        self
    }

    /// Add middleware to all routes in this group
    ///
    /// Middleware is applied in the order it's added.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// group!("/api", {
    ///     get!("/users", handler),
    /// }).middleware(AuthMiddleware).middleware(RateLimitMiddleware)
    /// ```
    pub fn middleware<M: Middleware + 'static>(mut self, middleware: M) -> Self {
        self.group_middlewares.push(into_boxed(middleware));
        self
    }

    /// Register all routes in this group with the router
    ///
    /// This prepends the group prefix to each route path and applies
    /// group middleware to all routes.
    ///
    /// # Path Combination
    ///
    /// - If route path is "/" (root), the full path is just the group prefix
    /// - Otherwise, prefix and route path are concatenated
    pub fn register(self, mut router: Router) -> Router {
        for route in self.routes {
            // Build full path with prefix
            // If route path is "/" (root), just use the prefix without trailing slash
            let full_path = if route.path == "/" {
                self.prefix.to_string()
            } else {
                format!("{}{}", self.prefix, route.path)
            };
            // We need to leak the string to get a 'static str for the router
            let full_path: &'static str = Box::leak(full_path.into_boxed_str());

            // Register the route with the router
            match route.method {
                HttpMethod::Get => {
                    router.insert_get(full_path, route.handler);
                }
                HttpMethod::Post => {
                    router.insert_post(full_path, route.handler);
                }
                HttpMethod::Put => {
                    router.insert_put(full_path, route.handler);
                }
                HttpMethod::Delete => {
                    router.insert_delete(full_path, route.handler);
                }
            }

            // Register route name if present
            if let Some(name) = route.name {
                register_route_name(name, full_path);
            }

            // Apply group middleware first (outer), then route-specific middleware (inner)
            for mw in &self.group_middlewares {
                router.add_middleware(full_path, mw.clone());
            }
            for mw in route.middlewares {
                router.add_middleware(full_path, mw);
            }
        }

        router
    }
}

impl<H, Fut> RouteDefBuilder<H>
where
    H: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    /// Convert this route definition to a type-erased GroupRoute
    ///
    /// This is used internally when adding routes to a group.
    pub fn into_group_route(self) -> GroupRoute {
        let handler = self.handler;
        let boxed: BoxedHandler = Box::new(move |req| Box::pin(handler(req)));
        GroupRoute {
            method: self.method,
            path: self.path,
            handler: Arc::new(boxed),
            name: self.name,
            middlewares: self.middlewares,
        }
    }
}

/// Define a route group with a shared prefix
///
/// Routes within a group will have the prefix prepended to their paths.
/// Middleware can be applied to the entire group using `.middleware()`.
///
/// # Example
///
/// ```rust,ignore
/// use kit::{routes, get, post, group};
///
/// routes! {
///     get!("/", controllers::home::index),
///
///     // All routes in this group start with /api
///     group!("/api", {
///         get!("/users", controllers::user::index),      // -> GET /api/users
///         post!("/users", controllers::user::store),     // -> POST /api/users
///     }).middleware(AuthMiddleware),
/// }
/// ```
/// Define a route group with a shared prefix and compile-time validation
///
/// Routes within a group will have the prefix prepended to their paths.
/// Middleware can be applied to the entire group using `.middleware()`.
///
/// # Compile Error
///
/// Fails to compile if prefix doesn't start with '/'.
#[macro_export]
macro_rules! group {
    ($prefix:expr, { $( $route:expr ),* $(,)? }) => {{
        const _: &str = $crate::validate_route_path($prefix);
        let mut group = $crate::GroupDef::__new_unchecked($prefix);
        $(
            group = group.route($route);
        )*
        group
    }};
}

/// Define routes with a clean, Laravel-like syntax
///
/// This macro generates a `pub fn register() -> Router` function automatically.
/// Place it at the top level of your `routes.rs` file.
///
/// # Example
/// ```rust,ignore
/// use kit::{routes, get, post, put, delete};
/// use crate::controllers;
/// use crate::middleware::AuthMiddleware;
///
/// routes! {
///     get!("/", controllers::home::index).name("home"),
///     get!("/users", controllers::user::index).name("users.index"),
///     get!("/users/{id}", controllers::user::show).name("users.show"),
///     post!("/users", controllers::user::store).name("users.store"),
///     put!("/users/{id}", controllers::user::update).name("users.update"),
///     delete!("/users/{id}", controllers::user::destroy).name("users.destroy"),
///     get!("/protected", controllers::home::index).middleware(AuthMiddleware),
/// }
/// ```
#[macro_export]
macro_rules! routes {
    ( $( $route:expr ),* $(,)? ) => {
        pub fn register() -> $crate::Router {
            let mut router = $crate::Router::new();
            $(
                router = $route.register(router);
            )*
            router
        }
    };
}
