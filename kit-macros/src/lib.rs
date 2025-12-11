//! Procedural macros for the Kit framework
//!
//! This crate provides compile-time validated macros for:
//! - Inertia.js responses with component validation
//! - Named route redirects with route validation
//! - Service auto-registration
//! - Handler attribute for controller methods
//! - FormRequest for validated request data

use proc_macro::TokenStream;

mod domain_error;
mod handler;
mod inertia;
mod injectable;
mod kit_test;
mod redirect;
mod request;
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

/// Define a domain error with automatic HTTP response conversion
///
/// This macro automatically:
/// 1. Derives `Debug` and `Clone` for the type
/// 2. Implements `Display`, `Error`, and `HttpError` traits
/// 3. Implements `From<T> for FrameworkError` for seamless `?` usage
///
/// # Attributes
///
/// - `status`: HTTP status code (default: 500)
/// - `message`: Error message for Display (default: struct name converted to sentence)
///
/// # Example
///
/// ```rust,ignore
/// use kit::domain_error;
///
/// #[domain_error(status = 404, message = "User not found")]
/// pub struct UserNotFoundError {
///     pub user_id: i32,
/// }
///
/// // Usage in controller - just use ? operator
/// pub async fn get_user(id: i32) -> Result<User, FrameworkError> {
///     users.find(id).ok_or(UserNotFoundError { user_id: id })?
/// }
/// ```
#[proc_macro_attribute]
pub fn domain_error(attr: TokenStream, input: TokenStream) -> TokenStream {
    domain_error::domain_error_impl(attr, input)
}

/// Attribute macro for controller handler methods
///
/// Transforms handler functions to automatically extract typed parameters
/// from HTTP requests using the `FromRequest` trait.
///
/// # Examples
///
/// ## With Request parameter:
/// ```rust,ignore
/// use kit::{handler, Request, Response, json_response};
///
/// #[handler]
/// pub async fn index(req: Request) -> Response {
///     json_response!({ "message": "Hello" })
/// }
/// ```
///
/// ## With FormRequest parameter:
/// ```rust,ignore
/// use kit::{handler, Response, json_response, request};
///
/// #[request]
/// pub struct CreateUserRequest {
///     #[validate(email)]
///     pub email: String,
/// }
///
/// #[handler]
/// pub async fn store(form: CreateUserRequest) -> Response {
///     // `form` is already validated - returns 422 if invalid
///     json_response!({ "email": form.email })
/// }
/// ```
///
/// ## Without parameters:
/// ```rust,ignore
/// #[handler]
/// pub async fn health_check() -> Response {
///     json_response!({ "status": "ok" })
/// }
/// ```
#[proc_macro_attribute]
pub fn handler(attr: TokenStream, input: TokenStream) -> TokenStream {
    handler::handler_impl(attr, input)
}

/// Derive macro for FormRequest trait
///
/// Generates the `FormRequest` trait implementation for a struct.
/// The struct must also derive `serde::Deserialize` and `validator::Validate`.
///
/// For the cleanest DX, use the `#[request]` attribute macro instead,
/// which handles all derives automatically.
///
/// # Example
///
/// ```rust,ignore
/// use kit::{FormRequest, Deserialize, Validate};
///
/// #[derive(Deserialize, Validate, FormRequest)]
/// pub struct CreateUserRequest {
///     #[validate(email)]
///     pub email: String,
///
///     #[validate(length(min = 8))]
///     pub password: String,
/// }
/// ```
#[proc_macro_derive(FormRequest)]
pub fn derive_form_request(input: TokenStream) -> TokenStream {
    request::derive_request_impl(input)
}

/// Attribute macro for clean request data definition
///
/// This is the recommended way to define validated request types.
/// It automatically adds the necessary derives and generates the trait impl.
///
/// Works with both:
/// - `application/json` - JSON request bodies
/// - `application/x-www-form-urlencoded` - HTML form submissions
///
/// # Example
///
/// ```rust,ignore
/// use kit::request;
///
/// #[request]
/// pub struct CreateUserRequest {
///     #[validate(email)]
///     pub email: String,
///
///     #[validate(length(min = 8))]
///     pub password: String,
/// }
///
/// // This can now be used directly in handlers:
/// #[handler]
/// pub async fn store(form: CreateUserRequest) -> Response {
///     // Automatically validated - returns 422 with errors if invalid
///     json_response!({ "email": form.email })
/// }
/// ```
#[proc_macro_attribute]
pub fn request(attr: TokenStream, input: TokenStream) -> TokenStream {
    request::request_attr_impl(attr, input)
}

/// Attribute macro for database-enabled tests
///
/// This macro simplifies writing tests that need database access by automatically
/// setting up an in-memory SQLite database with migrations applied.
///
/// By default, it uses `crate::migrations::Migrator` as the migrator type,
/// following Kit's convention for migration location.
///
/// # Examples
///
/// ## Basic usage (recommended):
/// ```rust,ignore
/// use kit::kit_test;
/// use kit::testing::TestDatabase;
///
/// #[kit_test]
/// async fn test_user_creation(db: TestDatabase) {
///     // db is an in-memory SQLite database with all migrations applied
///     // Any code using DB::connection() will use this test database
///     let action = CreateUserAction::new();
///     let user = action.execute("test@example.com").await.unwrap();
///     assert!(user.id > 0);
/// }
/// ```
///
/// ## Without TestDatabase parameter:
/// ```rust,ignore
/// #[kit_test]
/// async fn test_action_without_direct_db_access() {
///     // Database is set up but not directly accessed
///     // Actions using DB::connection() still work
///     let action = MyAction::new();
///     action.execute().await.unwrap();
/// }
/// ```
///
/// ## With custom migrator:
/// ```rust,ignore
/// #[kit_test(migrator = my_crate::CustomMigrator)]
/// async fn test_with_custom_migrator(db: TestDatabase) {
///     // Uses custom migrator instead of default
/// }
/// ```
#[proc_macro_attribute]
pub fn kit_test(attr: TokenStream, input: TokenStream) -> TokenStream {
    kit_test::kit_test_impl(attr, input)
}
