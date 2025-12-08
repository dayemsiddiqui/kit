//! Procedural macros for the Kit framework
//!
//! This crate provides compile-time validated macros for:
//! - Inertia.js responses with component validation
//! - Named route redirects with route validation
//! - Service auto-registration

use proc_macro::TokenStream;

mod inertia;
mod injectable;
mod redirect;
mod service;
mod utils;

/// Derive macro for generating `Serialize` implementation for Inertia props
///
/// # Example
///
/// ```rust,ignore
/// #[derive(InertiaProps)]
/// struct HomeProps {
///     title: String,
///     user: User,
/// }
/// ```
#[proc_macro_derive(InertiaProps)]
pub fn derive_inertia_props(input: TokenStream) -> TokenStream {
    inertia::derive_inertia_props_impl(input)
}

/// Create an Inertia response with compile-time component validation
///
/// # Examples
///
/// ## With typed struct (recommended for type safety):
/// ```rust,ignore
/// #[derive(InertiaProps)]
/// struct HomeProps {
///     title: String,
///     user: User,
/// }
///
/// inertia_response!("Home", HomeProps { title: "Welcome".into(), user })
/// ```
///
/// ## With JSON-like syntax (for quick prototyping):
/// ```rust,ignore
/// inertia_response!("Dashboard", { "user": { "name": "John" } })
/// ```
///
/// This macro validates that the component file exists at compile time.
/// If `frontend/src/pages/Dashboard.tsx` doesn't exist, you'll get a compile error.
#[proc_macro]
pub fn inertia_response(input: TokenStream) -> TokenStream {
    inertia::inertia_response_impl(input)
}

/// Create a redirect to a named route with compile-time validation
///
/// # Examples
///
/// ```rust,ignore
/// // Simple redirect
/// redirect!("users.index").into()
///
/// // Redirect with route parameters
/// redirect!("users.show").with("id", "42").into()
///
/// // Redirect with query parameters
/// redirect!("users.index").query("page", "1").into()
/// ```
///
/// This macro validates that the route name exists at compile time.
/// If the route doesn't exist, you'll get a compile error with suggestions.
#[proc_macro]
pub fn redirect(input: TokenStream) -> TokenStream {
    redirect::redirect_impl(input)
}

/// Mark a trait as a service for the App container
///
/// This attribute macro automatically adds `Send + Sync + 'static` bounds
/// to your trait, making it suitable for use with the dependency injection
/// container.
///
/// # Example
///
/// ```rust,ignore
/// use kit::service;
///
/// #[service]
/// pub trait HttpClient {
///     async fn get(&self, url: &str) -> Result<String, Error>;
/// }
///
/// // This expands to:
/// pub trait HttpClient: Send + Sync + 'static {
///     async fn get(&self, url: &str) -> Result<String, Error>;
/// }
/// ```
///
/// Then you can use it with the App container:
///
/// ```rust,ignore
/// // Register
/// App::bind::<dyn HttpClient>(Arc::new(RealHttpClient::new()));
///
/// // Resolve
/// let client: Arc<dyn HttpClient> = App::make::<dyn HttpClient>().unwrap();
/// ```
#[proc_macro_attribute]
pub fn service(attr: TokenStream, input: TokenStream) -> TokenStream {
    service::service_impl(attr, input)
}

/// Attribute macro to auto-register a concrete type as a singleton
///
/// This macro automatically:
/// 1. Derives `Default` and `Clone` for the struct
/// 2. Registers it as a singleton in the App container at startup
///
/// # Example
///
/// ```rust,ignore
/// use kit::injectable;
///
/// #[injectable]
/// pub struct AppState {
///     pub counter: u32,
/// }
///
/// // Automatically registered at startup
/// // Resolve via:
/// let state: AppState = App::get().unwrap();
/// ```
#[proc_macro_attribute]
pub fn injectable(_attr: TokenStream, input: TokenStream) -> TokenStream {
    injectable::injectable_impl(input)
}
