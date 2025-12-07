use kit::Router;

use crate::controllers;

pub fn register() -> Router {
    Router::new()
        .get("/", controllers::home::index)
        .into()
}
