use kit::{Request, Response};

pub async fn index(_req: Request) -> Response {
    Response::json(r#"{"users": [{"id": 1, "name": "John"}, {"id": 2, "name": "Jane"}]}"#)
}

pub async fn show(req: Request) -> Response {
    let id = req.param("id").map(|s| s.as_str()).unwrap_or("unknown");
    Response::json(format!(r#"{{"id": {}, "name": "User {}"}}"#, id, id))
}
