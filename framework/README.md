# Kit-RS

A Laravel-inspired web framework for Rust.

## Installation

Add Kit to your `Cargo.toml`:

```toml
[dependencies]
kit = { package = "kit-rs", version = "0.1" }
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use kit::{Router, Server, Request, Response};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .get("/", index)
        .get("/users/{id}", show_user);

    Server::new(router)
        .port(8080)
        .run()
        .await
        .expect("Failed to start server");
}

async fn index(_req: Request) -> Response {
    Response::text("Welcome to Kit!")
}

async fn show_user(req: Request) -> Response {
    let id = req.param("id").unwrap_or_default();
    Response::json(format!(r#"{{"id": "{}"}}"#, id))
}
```

## Features

- **Simple routing** - GET, POST, PUT, DELETE with route parameters
- **Async handlers** - Built on Tokio for high performance
- **Response builders** - Text, JSON, and custom responses
- **Laravel-inspired** - Familiar patterns for Laravel developers

## CLI Tool

Use the Kit CLI to scaffold new projects:

```bash
cargo install kit-cli
kit new myapp
```

## License

MIT
