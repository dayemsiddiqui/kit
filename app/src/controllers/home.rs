use kit::{Request, Response};

pub async fn index(_req: Request) -> Response {
    Response::text("Welcome to Kit!")
}
