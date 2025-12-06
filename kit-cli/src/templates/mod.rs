pub fn cargo_toml(package_name: &str, description: &str, author: &str) -> String {
    let authors_line = if author.is_empty() {
        String::new()
    } else {
        format!("authors = [\"{}\"]\n", author)
    };

    format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
description = "{}"
{}
[dependencies]
kit = {{ package = "kit-rs", version = "0.1" }}
tokio = {{ version = "1", features = ["full"] }}
"#,
        package_name, description, authors_line
    )
}

pub fn gitignore() -> &'static str {
    r#"/target
Cargo.lock
"#
}

pub fn main_rs() -> &'static str {
    r#"mod controllers;

use kit::{Router, Server};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .get("/", controllers::home::index);

    println!("Server running at http://localhost:8080");

    Server::new(router)
        .port(8080)
        .run()
        .await
        .expect("Failed to start server");
}
"#
}

pub fn controllers_mod() -> &'static str {
    "pub mod home;\n"
}

pub fn home_controller() -> &'static str {
    r#"use kit::{Request, Response};

pub async fn index(_req: Request) -> Response {
    Response::text("Welcome to Kit!")
}
"#
}
