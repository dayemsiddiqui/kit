//! Middleware registry for global middleware
//!
//! Configure global middleware in `bootstrap.rs` using the `global_middleware!` macro,
//! or use `Server::middleware()` for manual configuration.

use super::{into_boxed, BoxedMiddleware, Middleware};
use std::sync::{OnceLock, RwLock};

/// Global middleware registry (populated via `global_middleware!` macro in bootstrap.rs)
static GLOBAL_MIDDLEWARE: OnceLock<RwLock<Vec<BoxedMiddleware>>> = OnceLock::new();

/// Register a global middleware that runs on every request
///
/// Called by the `global_middleware!` macro. Middleware runs in registration order.
///
/// # Example
///
/// ```rust,ignore
/// // In bootstrap.rs
/// global_middleware!(LoggingMiddleware);
/// global_middleware!(CorsMiddleware);
/// ```
pub fn register_global_middleware<M: Middleware + 'static>(middleware: M) {
    let registry = GLOBAL_MIDDLEWARE.get_or_init(|| RwLock::new(Vec::new()));
    if let Ok(mut vec) = registry.write() {
        vec.push(into_boxed(middleware));
    }
}

/// Get all registered global middleware
///
/// Used internally by `Server::from_config()` to apply middleware.
pub fn get_global_middleware() -> Vec<BoxedMiddleware> {
    GLOBAL_MIDDLEWARE
        .get()
        .and_then(|lock| lock.read().ok())
        .map(|vec| vec.clone())
        .unwrap_or_default()
}

/// Registry for global middleware that runs on every request
///
/// # Example
///
/// ```rust,ignore
/// Server::from_config(router)
///     .middleware(LoggingMiddleware)  // Global middleware
///     .middleware(CorsMiddleware)
///     .run()
///     .await;
/// ```
pub struct MiddlewareRegistry {
    /// Middleware that runs on every request (in order)
    global: Vec<BoxedMiddleware>,
}

impl MiddlewareRegistry {
    /// Create a new empty middleware registry
    pub fn new() -> Self {
        Self { global: Vec::new() }
    }

    /// Create a registry pre-populated with globally registered middleware
    ///
    /// This pulls middleware registered via `global_middleware!` in bootstrap.rs.
    pub fn from_global() -> Self {
        Self {
            global: get_global_middleware(),
        }
    }

    /// Append global middleware that runs on every request
    ///
    /// Global middleware runs in the order they are added, before any
    /// route-specific middleware.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// m.append(LoggingMiddleware)
    ///  .append(CorsMiddleware)
    /// ```
    pub fn append<M: Middleware + 'static>(mut self, middleware: M) -> Self {
        self.global.push(into_boxed(middleware));
        self
    }

    /// Get the list of global middleware
    pub fn global_middleware(&self) -> &[BoxedMiddleware] {
        &self.global
    }
}

impl Default for MiddlewareRegistry {
    fn default() -> Self {
        Self::new()
    }
}
