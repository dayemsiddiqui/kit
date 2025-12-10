use crate::config::{Config, ServerConfig};
use crate::container::App;
use crate::http::{HttpResponse, Request};
use crate::inertia::InertiaContext;
use crate::middleware::{Middleware, MiddlewareChain, MiddlewareRegistry};
use crate::routing::Router;
use bytes::Bytes;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct Server {
    router: Arc<Router>,
    middleware: MiddlewareRegistry,
    host: String,
    port: u16,
}

impl Server {
    pub fn new(router: impl Into<Router>) -> Self {
        Self {
            router: Arc::new(router.into()),
            middleware: MiddlewareRegistry::new(),
            host: "127.0.0.1".to_string(),
            port: 8000,
        }
    }

    pub fn from_config(router: impl Into<Router>) -> Self {
        // Initialize the App container
        App::init();

        // Boot all auto-registered services from #[service(ConcreteType)]
        App::boot_services();

        let config = Config::get::<ServerConfig>().unwrap_or_else(ServerConfig::from_env);
        Self {
            router: Arc::new(router.into()),
            // Pull global middleware registered via global_middleware! in bootstrap.rs
            middleware: MiddlewareRegistry::from_global(),
            host: config.host,
            port: config.port,
        }
    }

    /// Add global middleware (runs on every request)
    ///
    /// For route-specific middleware, use `.middleware(M)` on the route itself.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// Server::from_config(router)
    ///     .middleware(LoggingMiddleware)  // Global
    ///     .middleware(CorsMiddleware)     // Global
    ///     .run()
    ///     .await;
    /// ```
    pub fn middleware<M: Middleware + 'static>(mut self, middleware: M) -> Self {
        self.middleware = self.middleware.append(middleware);
        self
    }

    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    fn get_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host.parse().unwrap(), self.port)
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr: SocketAddr = self.get_addr();
        let listener = TcpListener::bind(addr).await?;

        println!("Kit server running on http://{}", addr);

        let router = self.router;
        let middleware = Arc::new(self.middleware);

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let router = router.clone();
            let middleware = middleware.clone();

            tokio::spawn(async move {
                let service = service_fn(move |req: hyper::Request<hyper::body::Incoming>| {
                    let router = router.clone();
                    let middleware = middleware.clone();
                    async move { Ok::<_, Infallible>(handle_request(router, middleware, req).await) }
                });

                if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    }
}

async fn handle_request(
    router: Arc<Router>,
    middleware_registry: Arc<MiddlewareRegistry>,
    req: hyper::Request<hyper::body::Incoming>,
) -> hyper::Response<Full<Bytes>> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    // Set up Inertia context from request headers
    let is_inertia = req
        .headers()
        .get("X-Inertia")
        .and_then(|v| v.to_str().ok())
        .map(|v| v == "true")
        .unwrap_or(false);

    let inertia_version = req
        .headers()
        .get("X-Inertia-Version")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string());

    InertiaContext::set(InertiaContext {
        path: path.clone(),
        is_inertia,
        version: inertia_version,
    });

    let response = match router.match_route(&method, &path) {
        Some((handler, params)) => {
            let request = Request::new(req).with_params(params);

            // Build middleware chain
            let mut chain = MiddlewareChain::new();

            // 1. Add global middleware
            chain.extend(middleware_registry.global_middleware().iter().cloned());

            // 2. Add route-level middleware (already boxed)
            let route_middleware = router.get_route_middleware(&path);
            chain.extend(route_middleware);

            // 3. Execute chain with handler
            let response = chain.execute(request, handler).await;

            // Unwrap the Result - both Ok and Err contain HttpResponse
            let http_response = response.unwrap_or_else(|e| e);
            http_response.into_hyper()
        }
        None => HttpResponse::text("404 Not Found").status(404).into_hyper(),
    };

    // Clear context after request
    InertiaContext::clear();

    response
}
