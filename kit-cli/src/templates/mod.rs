pub fn cargo_toml(package_name: &str, description: &str, author: &str) -> String {
    let authors_line = if author.is_empty() {
        String::new()
    } else {
        format!("authors = [\"{}\"]\n", author)
    };

    format!(
        include_str!("files/Cargo.toml.tpl"),
        package_name = package_name,
        description = description,
        authors_line = authors_line
    )
}

pub fn gitignore() -> &'static str {
    include_str!("files/gitignore.tpl")
}

pub fn main_rs() -> &'static str {
    include_str!("files/main.rs.tpl")
}

pub fn controllers_mod() -> &'static str {
    include_str!("files/controllers_mod.rs.tpl")
}

pub fn home_controller() -> &'static str {
    include_str!("files/home_controller.rs.tpl")
}
