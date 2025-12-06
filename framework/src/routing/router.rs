use crate::http::{Request, Response};
use matchit::Router as MatchitRouter;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Type alias for route handlers
pub type BoxedHandler = Box<
    dyn Fn(Request) -> Pin<Box<dyn Future<Output = Response> + Send>> + Send + Sync,
>;

/// HTTP Router with Laravel-like route registration
pub struct Router {
    get_routes: MatchitRouter<Arc<BoxedHandler>>,
    post_routes: MatchitRouter<Arc<BoxedHandler>>,
    put_routes: MatchitRouter<Arc<BoxedHandler>>,
    delete_routes: MatchitRouter<Arc<BoxedHandler>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            get_routes: MatchitRouter::new(),
            post_routes: MatchitRouter::new(),
            put_routes: MatchitRouter::new(),
            delete_routes: MatchitRouter::new(),
        }
    }

    /// Register a GET route
    pub fn get<H, Fut>(mut self, path: &str, handler: H) -> Self
    where
        H: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        let handler: BoxedHandler = Box::new(move |req| Box::pin(handler(req)));
        self.get_routes.insert(path, Arc::new(handler)).ok();
        self
    }

    /// Register a POST route
    pub fn post<H, Fut>(mut self, path: &str, handler: H) -> Self
    where
        H: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        let handler: BoxedHandler = Box::new(move |req| Box::pin(handler(req)));
        self.post_routes.insert(path, Arc::new(handler)).ok();
        self
    }

    /// Register a PUT route
    pub fn put<H, Fut>(mut self, path: &str, handler: H) -> Self
    where
        H: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        let handler: BoxedHandler = Box::new(move |req| Box::pin(handler(req)));
        self.put_routes.insert(path, Arc::new(handler)).ok();
        self
    }

    /// Register a DELETE route
    pub fn delete<H, Fut>(mut self, path: &str, handler: H) -> Self
    where
        H: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        let handler: BoxedHandler = Box::new(move |req| Box::pin(handler(req)));
        self.delete_routes.insert(path, Arc::new(handler)).ok();
        self
    }

    /// Match a request and return the handler with extracted params
    pub fn match_route(
        &self,
        method: &hyper::Method,
        path: &str,
    ) -> Option<(Arc<BoxedHandler>, HashMap<String, String>)> {
        let router = match *method {
            hyper::Method::GET => &self.get_routes,
            hyper::Method::POST => &self.post_routes,
            hyper::Method::PUT => &self.put_routes,
            hyper::Method::DELETE => &self.delete_routes,
            _ => return None,
        };

        router.at(path).ok().map(|matched| {
            let params: HashMap<String, String> = matched
                .params
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            (matched.value.clone(), params)
        })
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
