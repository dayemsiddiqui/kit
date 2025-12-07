use kit::{inertia_response, Request, Response};

pub async fn index(_req: Request) -> Response {
    inertia_response!("Home", {
        "title": "Welcome to Kit!",
        "user": {
            "name": "John Doe",
            "email": "john@example.com"
        },
        "stats": {
            "visits": 1234,
            "likes": 567
        }
    })
}
