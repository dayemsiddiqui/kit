mod controllers;

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
