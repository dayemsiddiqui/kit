use super::ParamError;
use std::collections::HashMap;

/// HTTP Request wrapper providing Laravel-like access to request data
pub struct Request {
    inner: hyper::Request<hyper::body::Incoming>,
    params: HashMap<String, String>,
}

impl Request {
    pub fn new(inner: hyper::Request<hyper::body::Incoming>) -> Self {
        Self {
            inner,
            params: HashMap::new(),
        }
    }

    pub fn with_params(mut self, params: HashMap<String, String>) -> Self {
        self.params = params;
        self
    }

    /// Get the request method
    pub fn method(&self) -> &hyper::Method {
        self.inner.method()
    }

    /// Get the request path
    pub fn path(&self) -> &str {
        self.inner.uri().path()
    }

    /// Get a route parameter by name (e.g., /users/{id})
    /// Returns Err(ParamError) if the parameter is missing, enabling use of `?` operator
    pub fn param(&self, name: &str) -> Result<&str, ParamError> {
        self.params
            .get(name)
            .map(|s| s.as_str())
            .ok_or_else(|| ParamError {
                param_name: name.to_string(),
            })
    }

    /// Get all route parameters
    pub fn params(&self) -> &HashMap<String, String> {
        &self.params
    }

    /// Get the inner hyper request
    pub fn inner(&self) -> &hyper::Request<hyper::body::Incoming> {
        &self.inner
    }

    /// Get a header value by name
    pub fn header(&self, name: &str) -> Option<&str> {
        self.inner
            .headers()
            .get(name)
            .and_then(|v| v.to_str().ok())
    }

    /// Check if this is an Inertia XHR request
    pub fn is_inertia(&self) -> bool {
        self.header("X-Inertia")
            .map(|v| v == "true")
            .unwrap_or(false)
    }

    /// Get the Inertia version from request headers
    pub fn inertia_version(&self) -> Option<&str> {
        self.header("X-Inertia-Version")
    }

    /// Get partial component name for partial reloads
    pub fn inertia_partial_component(&self) -> Option<&str> {
        self.header("X-Inertia-Partial-Component")
    }

    /// Get partial data keys for partial reloads
    pub fn inertia_partial_data(&self) -> Option<Vec<&str>> {
        self.header("X-Inertia-Partial-Data")
            .map(|v| v.split(',').collect())
    }
}
