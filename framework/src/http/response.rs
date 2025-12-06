use bytes::Bytes;
use http_body_util::Full;

/// HTTP Response builder providing Laravel-like response creation
pub struct Response {
    status: u16,
    body: String,
    headers: Vec<(String, String)>,
}

impl Response {
    pub fn new() -> Self {
        Self {
            status: 200,
            body: String::new(),
            headers: Vec::new(),
        }
    }

    /// Create a response with a string body
    pub fn text(body: impl Into<String>) -> Self {
        Self {
            status: 200,
            body: body.into(),
            headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
        }
    }

    /// Create a JSON response
    pub fn json(body: impl Into<String>) -> Self {
        Self {
            status: 200,
            body: body.into(),
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
        }
    }

    /// Set the HTTP status code
    pub fn status(mut self, status: u16) -> Self {
        self.status = status;
        self
    }

    /// Add a header to the response
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    /// Convert to hyper response
    pub fn into_hyper(self) -> hyper::Response<Full<Bytes>> {
        let mut builder = hyper::Response::builder().status(self.status);

        for (name, value) in self.headers {
            builder = builder.header(name, value);
        }

        builder
            .body(Full::new(Bytes::from(self.body)))
            .unwrap()
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
}
