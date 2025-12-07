// Backend templates

pub fn cargo_toml(package_name: &str, description: &str, author: &str) -> String {
    let authors_line = if author.is_empty() {
        String::new()
    } else {
        format!("authors = [\"{}\"]\n", author)
    };

    format!(
        include_str!("files/backend/Cargo.toml.tpl"),
        package_name = package_name,
        description = description,
        authors_line = authors_line
    )
}

pub fn main_rs() -> &'static str {
    include_str!("files/backend/main.rs.tpl")
}

pub fn routes_rs() -> &'static str {
    include_str!("files/backend/routes.rs.tpl")
}

pub fn controllers_mod() -> &'static str {
    include_str!("files/backend/controllers/mod.rs.tpl")
}

pub fn home_controller() -> &'static str {
    include_str!("files/backend/controllers/home.rs.tpl")
}

// Frontend templates

pub fn package_json(project_name: &str) -> String {
    include_str!("files/frontend/package.json.tpl").replace("{project_name}", project_name)
}

pub fn vite_config() -> &'static str {
    include_str!("files/frontend/vite.config.ts.tpl")
}

pub fn tsconfig() -> &'static str {
    include_str!("files/frontend/tsconfig.json.tpl")
}

pub fn index_html(project_title: &str) -> String {
    include_str!("files/frontend/index.html.tpl").replace("{project_title}", project_title)
}

pub fn main_tsx() -> &'static str {
    include_str!("files/frontend/src/main.tsx.tpl")
}

pub fn home_page() -> &'static str {
    include_str!("files/frontend/src/pages/Home.tsx.tpl")
}

pub fn inertia_props_types() -> &'static str {
    include_str!("files/frontend/src/types/inertia-props.ts.tpl")
}

// Root templates

pub fn gitignore() -> &'static str {
    include_str!("files/root/gitignore.tpl")
}
