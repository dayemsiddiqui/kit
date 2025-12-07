use kit::{text, Request, Response};

pub async fn index(_req: Request) -> Response {
    text("Welcome to Kit!")
}
