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
    pub fn param(&self, name: &str) -> Option<&String> {
        self.params.get(name)
    }

    /// Get all route parameters
    pub fn params(&self) -> &HashMap<String, String> {
        &self.params
    }

    /// Get the inner hyper request
    pub fn inner(&self) -> &hyper::Request<hyper::body::Incoming> {
        &self.inner
    }
}
