use kit::{Router, Server};

mod controllers;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .get("/", controllers::home::index)
        .get("/users", controllers::user::index)
        .get("/users/{id}", controllers::user::show);

    Server::new(router)
        .run()
        .await
        .expect("Failed to start server");
}
