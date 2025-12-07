use kit::Router;

use crate::controllers;

pub fn register() -> Router {
    Router::new()
        .get("/", controllers::home::index).name("home")
        .get("/users", controllers::user::index).name("users.index")
        .get("/users/{id}", controllers::user::show).name("users.show")
        .post("/users", controllers::user::store).name("users.store")
        .get("/redirect-example", controllers::user::redirect_example)
        .get("/config", controllers::config_example::show).name("config.show")
        .into()
}
