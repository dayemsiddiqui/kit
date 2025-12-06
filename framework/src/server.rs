use crate::http::{Request, Response};
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

/// HTTP Server that runs your Kit application
pub struct Server {
    router: Arc<Router>,
    host: String,
    port: u16,
}

impl Server {
    pub fn new(router: Router) -> Self {
        Self {
            router: Arc::new(router),
            host: "127.0.0.1".to_string(),
            port: 8000,
        }
    }
    
    pub fn host(&mut self, host: &str) -> &mut Self {
        self.host = host.to_string();
        self
    }
    
    pub fn port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }
    
    fn get_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host.parse().unwrap(), self.port)
    }

    /// Start the server on the given address
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Default to localhost:8000
        let addr: SocketAddr = self.get_addr();
        let listener = TcpListener::bind(addr).await?;

        println!("Kit server running on http://{}", addr);

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let router = self.router.clone();

            tokio::spawn(async move {
                let service = service_fn(move |req: hyper::Request<hyper::body::Incoming>| {
                    let router = router.clone();
                    async move { Ok::<_, Infallible>(handle_request(router, req).await) }
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
    req: hyper::Request<hyper::body::Incoming>,
) -> hyper::Response<Full<Bytes>> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    match router.match_route(&method, &path) {
        Some((handler, params)) => {
            let request = Request::new(req).with_params(params);
            let response = handler(request).await;
            response.into_hyper()
        }
        None => {
            Response::text("404 Not Found")
                .status(404)
                .into_hyper()
        }
    }
}
