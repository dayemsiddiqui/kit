use console::style;
use dialoguer::{Input, theme::ColorfulTheme};
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::templates;

pub fn run(name: Option<String>, no_interaction: bool, no_git: bool) {
    println!();
    println!("{}", style("Welcome to Kit!").cyan().bold());
    println!();

    let project_name = get_project_name(name, no_interaction);
    let description = get_description(no_interaction);
    let author = get_author(no_interaction);

    let package_name = to_snake_case(&project_name);

    println!();
    println!(
        "{}",
        style(format!("Creating project '{}'...", project_name)).dim()
    );

    if let Err(e) = create_project(&project_name, &package_name, &description, &author, no_git) {
        eprintln!("{} {}", style("Error:").red().bold(), e);
        std::process::exit(1);
    }

    println!("{} Generated project structure", style("✓").green());

    if !no_git {
        println!("{} Initialized git repository", style("✓").green());
    }

    println!("{} Ready to go!", style("✓").green());
    println!();
    println!("Next steps:");
    println!("  {} {}", style("cd").cyan(), project_name);
    println!("  {}", style("kit serve").cyan());
    println!();
    println!(
        "Backend will be at {}",
        style("http://localhost:8000").underlined()
    );
    println!(
        "Frontend dev server at {}",
        style("http://localhost:5173").underlined()
    );
    println!();
}

fn get_project_name(name: Option<String>, no_interaction: bool) -> String {
    if let Some(n) = name {
        return n;
    }

    if no_interaction {
        return "my-kit-app".to_string();
    }

    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project name")
        .default("my-kit-app".to_string())
        .interact_text()
        .unwrap()
}

fn get_description(no_interaction: bool) -> String {
    if no_interaction {
        return "A web application built with Kit".to_string();
    }

    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Description")
        .default("A web application built with Kit".to_string())
        .interact_text()
        .unwrap()
}

fn get_author(no_interaction: bool) -> String {
    if no_interaction {
        return String::new();
    }

    let default_author = get_git_author().unwrap_or_default();

    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Author")
        .default(default_author)
        .allow_empty(true)
        .interact_text()
        .unwrap()
}

fn get_git_author() -> Option<String> {
    let name = Command::new("git")
        .args(["config", "user.name"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())?;

    let email = Command::new("git")
        .args(["config", "user.email"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())?;

    Some(format!("{} <{}>", name, email))
}

fn to_snake_case(s: &str) -> String {
    s.replace('-', "_").to_lowercase()
}

fn to_title_case(s: &str) -> String {
    s.replace('-', " ")
        .replace('_', " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn create_project(
    project_name: &str,
    package_name: &str,
    description: &str,
    author: &str,
    no_git: bool,
) -> Result<(), String> {
    let project_path = Path::new(project_name);

    if project_path.exists() {
        return Err(format!("Directory '{}' already exists", project_name));
    }

    // Create directory structure
    // Backend directories
    fs::create_dir_all(project_path.join("src/controllers"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;

    // Frontend directories
    fs::create_dir_all(project_path.join("frontend/src/pages"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;
    fs::create_dir_all(project_path.join("frontend/src/types"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;

    // Public assets directory (for production builds)
    fs::create_dir_all(project_path.join("public/assets"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;

    // === Backend files ===

    // Write Cargo.toml
    let cargo_toml = templates::cargo_toml(package_name, description, author);
    fs::write(project_path.join("Cargo.toml"), cargo_toml)
        .map_err(|e| format!("Failed to write Cargo.toml: {}", e))?;

    // Write .gitignore
    fs::write(project_path.join(".gitignore"), templates::gitignore())
        .map_err(|e| format!("Failed to write .gitignore: {}", e))?;

    // Write src/main.rs
    fs::write(project_path.join("src/main.rs"), templates::main_rs())
        .map_err(|e| format!("Failed to write src/main.rs: {}", e))?;

    // Write src/controllers/mod.rs
    fs::write(
        project_path.join("src/controllers/mod.rs"),
        templates::controllers_mod(),
    )
    .map_err(|e| format!("Failed to write src/controllers/mod.rs: {}", e))?;

    // Write src/controllers/home.rs
    fs::write(
        project_path.join("src/controllers/home.rs"),
        templates::home_controller(),
    )
    .map_err(|e| format!("Failed to write src/controllers/home.rs: {}", e))?;

    // === Frontend files ===

    // Write frontend/package.json
    let package_json = templates::package_json(project_name);
    fs::write(project_path.join("frontend/package.json"), package_json)
        .map_err(|e| format!("Failed to write frontend/package.json: {}", e))?;

    // Write frontend/vite.config.ts
    fs::write(
        project_path.join("frontend/vite.config.ts"),
        templates::vite_config(),
    )
    .map_err(|e| format!("Failed to write frontend/vite.config.ts: {}", e))?;

    // Write frontend/tsconfig.json
    fs::write(
        project_path.join("frontend/tsconfig.json"),
        templates::tsconfig(),
    )
    .map_err(|e| format!("Failed to write frontend/tsconfig.json: {}", e))?;

    // Write frontend/index.html
    let title = to_title_case(project_name);
    let index_html = templates::index_html(&title);
    fs::write(project_path.join("frontend/index.html"), index_html)
        .map_err(|e| format!("Failed to write frontend/index.html: {}", e))?;

    // Write frontend/src/main.tsx
    fs::write(
        project_path.join("frontend/src/main.tsx"),
        templates::main_tsx(),
    )
    .map_err(|e| format!("Failed to write frontend/src/main.tsx: {}", e))?;

    // Write frontend/src/pages/Home.tsx
    fs::write(
        project_path.join("frontend/src/pages/Home.tsx"),
        templates::home_page(),
    )
    .map_err(|e| format!("Failed to write frontend/src/pages/Home.tsx: {}", e))?;

    // Write frontend/src/types/inertia-props.ts
    fs::write(
        project_path.join("frontend/src/types/inertia-props.ts"),
        templates::inertia_props_types(),
    )
    .map_err(|e| format!("Failed to write frontend/src/types/inertia-props.ts: {}", e))?;

    // Initialize git repository
    if !no_git {
        Command::new("git")
            .args(["init"])
            .current_dir(project_path)
            .output()
            .map_err(|e| format!("Failed to initialize git repository: {}", e))?;
    }

    Ok(())
}
