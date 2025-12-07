mod controllers;
mod routes;

use kit::Server;

#[tokio::main]
async fn main() {
    let router = routes::register();

    println!("Server running at http://localhost:8080");

    Server::new(router)
        .port(8080)
        .run()
        .await
        .expect("Failed to start server");
}
