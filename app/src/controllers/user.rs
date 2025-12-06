use kit::{json_response, Request, Response};

pub async fn index(_req: Request) -> Response {
    json_response!({
        "users": [
            {"id": 1, "name": "John"},
            {"id": 2, "name": "Jane"}
        ]
    })
}

pub async fn show(req: Request) -> Response {
    let id = req.param("id").map(|s| s.as_str()).unwrap_or("unknown");
    json_response!({
        "id": id,
        "name": format!("User {}", id)
    })
}
