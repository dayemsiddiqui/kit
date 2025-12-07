use kit::{inertia_response, InertiaProps, Request, Response};

#[derive(InertiaProps)]
pub struct User {
    pub name: String,
    pub email: String,
}

#[derive(InertiaProps)]
pub struct Stats {
    pub visits: u32,
    pub likes: u32,
}

#[derive(InertiaProps)]
pub struct HomeProps {
    pub title: String,
    pub user: User,
    pub stats: Stats,
}

pub async fn index(_req: Request) -> Response {
    inertia_response!("Home", HomeProps {
        title: "Welcome to Kit!".to_string(),
        user: User {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        },
        stats: Stats {
            visits: 1234,
            likes: 567,
        },
    })
}
