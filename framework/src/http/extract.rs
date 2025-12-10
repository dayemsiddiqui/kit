//! Request extraction traits for handler parameter injection
//!
//! This module provides the `FromRequest` trait which enables the `#[handler]`
//! macro to automatically extract typed parameters from incoming requests.

use super::Request;
use crate::error::FrameworkError;
use async_trait::async_trait;

/// Trait for types that can be extracted from an HTTP request
///
/// This trait is used by the `#[handler]` macro to automatically
/// extract and inject typed parameters into controller handlers.
///
/// # Implementations
///
/// - `Request` - passes the request through unchanged
/// - Any type implementing `FormRequest` - automatically parses and validates
///
/// # Example
///
/// The `#[handler]` macro uses this trait to transform:
///
/// ```rust,ignore
/// #[handler]
/// pub async fn store(form: CreateUserRequest) -> Response {
///     // ...
/// }
/// ```
///
/// Into:
///
/// ```rust,ignore
/// pub async fn store(req: Request) -> Response {
///     let form = <CreateUserRequest as FromRequest>::from_request(req).await?;
///     // ...
/// }
/// ```
#[async_trait]
pub trait FromRequest: Sized + Send {
    /// Extract Self from the incoming request
    ///
    /// Returns `Err(FrameworkError)` if extraction fails, which will be
    /// converted to an appropriate HTTP error response.
    async fn from_request(req: Request) -> Result<Self, FrameworkError>;
}

/// Request passes through unchanged
#[async_trait]
impl FromRequest for Request {
    async fn from_request(req: Request) -> Result<Self, FrameworkError> {
        Ok(req)
    }
}
