//! Application Container for Dependency Injection
//!
//! This module provides Laravel-like service container capabilities:
//! - Singletons: shared instances across the application
//! - Factories: new instance per resolution
//! - Trait bindings: bind interfaces to implementations
//! - Test faking: swap implementations in tests
//! - Service Providers: bootstrap services with register/boot lifecycle
//!
//! # Example
//!
//! ```rust,ignore
//! use kit::{App, bind, singleton, service};
//!
//! // Define a service trait with auto-registration
//! #[service(RealHttpClient)]
//! pub trait HttpClient {
//!     async fn get(&self, url: &str) -> Result<String, Error>;
//! }
//!
//! // Or register manually using macros
//! bind!(dyn HttpClient, RealHttpClient::new());
//! singleton!(CacheService::new());
//!
//! // Resolve anywhere in your app
//! let client: Arc<dyn HttpClient> = App::make::<dyn HttpClient>().unwrap();
//! ```

pub mod provider;
pub mod testing;

use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

/// Global application container
static APP_CONTAINER: OnceLock<RwLock<Container>> = OnceLock::new();

// Thread-local test overrides for isolated testing
thread_local! {
    pub(crate) static TEST_CONTAINER: RefCell<Option<Container>> = const { RefCell::new(None) };
}

/// Binding types: either a singleton instance or a factory closure
#[derive(Clone)]
enum Binding {
    /// Shared singleton instance - same instance returned every time
    Singleton(Arc<dyn Any + Send + Sync>),

    /// Factory closure - creates new instance each time
    Factory(Arc<dyn Fn() -> Arc<dyn Any + Send + Sync> + Send + Sync>),
}

/// The main service container
///
/// Stores type-erased bindings keyed by TypeId. Supports both concrete types
/// and trait objects (via Arc<dyn Trait>).
pub struct Container {
    /// Type bindings: TypeId -> Binding
    bindings: HashMap<TypeId, Binding>,
}

impl Container {
    /// Create a new empty container
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    /// Register a singleton instance (shared across all resolutions)
    ///
    /// # Example
    /// ```rust,ignore
    /// container.singleton(DatabaseConnection::new(&url));
    /// ```
    pub fn singleton<T: Any + Send + Sync + 'static>(&mut self, instance: T) {
        let arc: Arc<dyn Any + Send + Sync> = Arc::new(instance);
        self.bindings
            .insert(TypeId::of::<T>(), Binding::Singleton(arc));
    }

    /// Register a factory closure (new instance per resolution)
    ///
    /// # Example
    /// ```rust,ignore
    /// container.factory(|| RequestLogger::new());
    /// ```
    pub fn factory<T, F>(&mut self, factory: F)
    where
        T: Any + Send + Sync + 'static,
        F: Fn() -> T + Send + Sync + 'static,
    {
        let wrapped: Arc<dyn Fn() -> Arc<dyn Any + Send + Sync> + Send + Sync> =
            Arc::new(move || Arc::new(factory()) as Arc<dyn Any + Send + Sync>);
        self.bindings
            .insert(TypeId::of::<T>(), Binding::Factory(wrapped));
    }

    /// Bind a trait object to a concrete implementation (as singleton)
    ///
    /// This stores the value under `TypeId::of::<Arc<dyn Trait>>()` which allows
    /// trait objects to be resolved via `make::<dyn Trait>()`.
    ///
    /// # Example
    /// ```rust,ignore
    /// container.bind::<dyn HttpClient>(RealHttpClient::new());
    /// ```
    pub fn bind<T: ?Sized + Send + Sync + 'static>(&mut self, instance: Arc<T>) {
        // Store under TypeId of Arc<T> (works for both concrete and trait objects)
        let type_id = TypeId::of::<Arc<T>>();
        let arc: Arc<dyn Any + Send + Sync> = Arc::new(instance);
        self.bindings.insert(type_id, Binding::Singleton(arc));
    }

    /// Bind a trait object to a factory
    ///
    /// # Example
    /// ```rust,ignore
    /// container.bind_factory::<dyn HttpClient>(|| Arc::new(RealHttpClient::new()));
    /// ```
    pub fn bind_factory<T: ?Sized + Send + Sync + 'static, F>(&mut self, factory: F)
    where
        F: Fn() -> Arc<T> + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<Arc<T>>();
        let wrapped: Arc<dyn Fn() -> Arc<dyn Any + Send + Sync> + Send + Sync> =
            Arc::new(move || Arc::new(factory()) as Arc<dyn Any + Send + Sync>);
        self.bindings.insert(type_id, Binding::Factory(wrapped));
    }

    /// Resolve a concrete type (requires Clone)
    ///
    /// # Example
    /// ```rust,ignore
    /// let db: DatabaseConnection = container.get().unwrap();
    /// ```
    pub fn get<T: Any + Send + Sync + Clone + 'static>(&self) -> Option<T> {
        match self.bindings.get(&TypeId::of::<T>())? {
            Binding::Singleton(arc) => arc.downcast_ref::<T>().cloned(),
            Binding::Factory(factory) => {
                let arc = factory();
                arc.downcast_ref::<T>().cloned()
            }
        }
    }

    /// Resolve a trait binding - returns Arc<T>
    ///
    /// # Example
    /// ```rust,ignore
    /// let client: Arc<dyn HttpClient> = container.make::<dyn HttpClient>().unwrap();
    /// ```
    pub fn make<T: ?Sized + Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        let type_id = TypeId::of::<Arc<T>>();
        match self.bindings.get(&type_id)? {
            Binding::Singleton(arc) => {
                // The stored value is Arc<Arc<T>>, so we downcast and clone the inner Arc
                arc.downcast_ref::<Arc<T>>().cloned()
            }
            Binding::Factory(factory) => {
                let arc = factory();
                arc.downcast_ref::<Arc<T>>().cloned()
            }
        }
    }

    /// Check if a concrete type is registered
    pub fn has<T: Any + 'static>(&self) -> bool {
        self.bindings.contains_key(&TypeId::of::<T>())
    }

    /// Check if a trait binding is registered
    pub fn has_binding<T: ?Sized + 'static>(&self) -> bool {
        self.bindings.contains_key(&TypeId::of::<Arc<T>>())
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

/// Application container facade
///
/// Provides static methods for service registration and resolution.
/// Uses a global container with thread-local test overrides.
///
/// # Example
///
/// ```rust,ignore
/// use kit::{App, bind, singleton};
///
/// // Register services at startup using macros
/// singleton!(DatabaseConnection::new(&url));
/// bind!(dyn HttpClient, RealHttpClient::new());
///
/// // Resolve anywhere
/// let db: DatabaseConnection = App::get().unwrap();
/// let client: Arc<dyn HttpClient> = App::make::<dyn HttpClient>().unwrap();
/// ```
pub struct App;

impl App {
    /// Initialize the application container
    ///
    /// Should be called once at application startup. This is automatically
    /// called by `Server::from_config()`.
    pub fn init() {
        APP_CONTAINER.get_or_init(|| RwLock::new(Container::new()));
    }

    /// Register a singleton instance (shared across all resolutions)
    ///
    /// # Example
    /// ```rust,ignore
    /// App::singleton(DatabaseConnection::new(&url));
    /// ```
    pub fn singleton<T: Any + Send + Sync + 'static>(instance: T) {
        let container = APP_CONTAINER.get_or_init(|| RwLock::new(Container::new()));
        if let Ok(mut c) = container.write() {
            c.singleton(instance);
        }
    }

    /// Register a factory binding (new instance per resolution)
    ///
    /// # Example
    /// ```rust,ignore
    /// App::factory(|| RequestLogger::new());
    /// ```
    pub fn factory<T, F>(factory: F)
    where
        T: Any + Send + Sync + 'static,
        F: Fn() -> T + Send + Sync + 'static,
    {
        let container = APP_CONTAINER.get_or_init(|| RwLock::new(Container::new()));
        if let Ok(mut c) = container.write() {
            c.factory(factory);
        }
    }

    /// Bind a trait object to a concrete implementation (as singleton)
    ///
    /// # Example
    /// ```rust,ignore
    /// App::bind::<dyn HttpClient>(Arc::new(RealHttpClient::new()));
    /// ```
    pub fn bind<T: ?Sized + Send + Sync + 'static>(instance: Arc<T>) {
        let container = APP_CONTAINER.get_or_init(|| RwLock::new(Container::new()));
        if let Ok(mut c) = container.write() {
            c.bind(instance);
        }
    }

    /// Bind a trait object to a factory
    ///
    /// # Example
    /// ```rust,ignore
    /// App::bind_factory::<dyn HttpClient>(|| Arc::new(RealHttpClient::new()));
    /// ```
    pub fn bind_factory<T: ?Sized + Send + Sync + 'static, F>(factory: F)
    where
        F: Fn() -> Arc<T> + Send + Sync + 'static,
    {
        let container = APP_CONTAINER.get_or_init(|| RwLock::new(Container::new()));
        if let Ok(mut c) = container.write() {
            c.bind_factory(factory);
        }
    }

    /// Resolve a concrete type
    ///
    /// Checks test overrides first, then falls back to global container.
    ///
    /// # Example
    /// ```rust,ignore
    /// let db: DatabaseConnection = App::get().unwrap();
    /// ```
    pub fn get<T: Any + Send + Sync + Clone + 'static>() -> Option<T> {
        // Check test overrides first (thread-local)
        let test_result = TEST_CONTAINER.with(|c| {
            c.borrow()
                .as_ref()
                .and_then(|container| container.get::<T>())
        });

        if test_result.is_some() {
            return test_result;
        }

        // Fall back to global container
        let container = APP_CONTAINER.get()?;
        container.read().ok()?.get::<T>()
    }

    /// Resolve a trait binding - returns Arc<T>
    ///
    /// Checks test overrides first, then falls back to global container.
    ///
    /// # Example
    /// ```rust,ignore
    /// let client: Arc<dyn HttpClient> = App::make::<dyn HttpClient>().unwrap();
    /// ```
    pub fn make<T: ?Sized + Send + Sync + 'static>() -> Option<Arc<T>> {
        // Check test overrides first (thread-local)
        let test_result = TEST_CONTAINER.with(|c| {
            c.borrow()
                .as_ref()
                .and_then(|container| container.make::<T>())
        });

        if test_result.is_some() {
            return test_result;
        }

        // Fall back to global container
        let container = APP_CONTAINER.get()?;
        container.read().ok()?.make::<T>()
    }

    /// Resolve a concrete type, returning an error if not found
    ///
    /// This allows using the `?` operator in controllers and services for
    /// automatic error propagation with proper HTTP responses.
    ///
    /// # Example
    /// ```rust,ignore
    /// pub async fn index(_req: Request) -> Response {
    ///     let service = App::resolve::<MyService>()?;
    ///     // ...
    /// }
    /// ```
    pub fn resolve<T: Any + Send + Sync + Clone + 'static>(
    ) -> Result<T, crate::error::FrameworkError> {
        Self::get::<T>().ok_or_else(crate::error::FrameworkError::service_not_found::<T>)
    }

    /// Resolve a trait binding, returning an error if not found
    ///
    /// This allows using the `?` operator for trait object resolution.
    ///
    /// # Example
    /// ```rust,ignore
    /// let client: Arc<dyn HttpClient> = App::resolve_make::<dyn HttpClient>()?;
    /// ```
    pub fn resolve_make<T: ?Sized + Send + Sync + 'static>(
    ) -> Result<Arc<T>, crate::error::FrameworkError> {
        Self::make::<T>().ok_or_else(crate::error::FrameworkError::service_not_found::<T>)
    }

    /// Check if a concrete type is registered
    pub fn has<T: Any + 'static>() -> bool {
        // Check test container first
        let in_test = TEST_CONTAINER.with(|c| {
            c.borrow()
                .as_ref()
                .map(|container| container.has::<T>())
                .unwrap_or(false)
        });

        if in_test {
            return true;
        }

        APP_CONTAINER
            .get()
            .and_then(|c| c.read().ok())
            .map(|c| c.has::<T>())
            .unwrap_or(false)
    }

    /// Check if a trait binding is registered
    pub fn has_binding<T: ?Sized + 'static>() -> bool {
        // Check test container first
        let in_test = TEST_CONTAINER.with(|c| {
            c.borrow()
                .as_ref()
                .map(|container| container.has_binding::<T>())
                .unwrap_or(false)
        });

        if in_test {
            return true;
        }

        APP_CONTAINER
            .get()
            .and_then(|c| c.read().ok())
            .map(|c| c.has_binding::<T>())
            .unwrap_or(false)
    }

    /// Boot all auto-registered services
    ///
    /// This registers all services marked with `#[service(ConcreteType)]`.
    /// Called automatically by `Server::from_config()`.
    pub fn boot_services() {
        provider::bootstrap();
    }
}

/// Bind a trait to a singleton implementation (auto-wraps in Arc)
///
/// # Example
/// ```rust,ignore
/// bind!(dyn Database, PostgresDB::connect(&db_url));
/// bind!(dyn HttpClient, RealHttpClient::new());
/// ```
#[macro_export]
macro_rules! bind {
    ($trait:ty, $instance:expr) => {
        $crate::App::bind::<$trait>(::std::sync::Arc::new($instance) as ::std::sync::Arc<$trait>)
    };
}

/// Bind a trait to a factory (auto-wraps in Arc, new instance each resolution)
///
/// # Example
/// ```rust,ignore
/// bind_factory!(dyn HttpClient, || RealHttpClient::new());
/// ```
#[macro_export]
macro_rules! bind_factory {
    ($trait:ty, $factory:expr) => {{
        let f = $factory;
        $crate::App::bind_factory::<$trait, _>(move || {
            ::std::sync::Arc::new(f()) as ::std::sync::Arc<$trait>
        })
    }};
}

/// Register a singleton instance (concrete type)
///
/// # Example
/// ```rust,ignore
/// singleton!(DatabaseConnection::new(&url));
/// ```
#[macro_export]
macro_rules! singleton {
    ($instance:expr) => {
        $crate::App::singleton($instance)
    };
}

/// Register a factory (concrete type, new instance each resolution)
///
/// # Example
/// ```rust,ignore
/// factory!(|| RequestLogger::new());
/// ```
#[macro_export]
macro_rules! factory {
    ($factory:expr) => {
        $crate::App::factory($factory)
    };
}
