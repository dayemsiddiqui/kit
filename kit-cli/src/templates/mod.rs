// Types for entity generation templates

/// Column information from database schema
pub struct ColumnInfo {
    pub name: String,
    pub col_type: String,
    pub is_nullable: bool,
    pub is_primary_key: bool,
}

/// Table information from database schema
pub struct TableInfo {
    pub name: String,
    pub columns: Vec<ColumnInfo>,
}

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

// Middleware templates

pub fn middleware_mod() -> &'static str {
    include_str!("files/backend/middleware/mod.rs.tpl")
}

pub fn middleware_logging() -> &'static str {
    include_str!("files/backend/middleware/logging.rs.tpl")
}

/// Template for generating new middleware with make:middleware command
pub fn middleware_template(name: &str, struct_name: &str) -> String {
    format!(
        r#"//! {name} middleware

use kit::{{async_trait, Middleware, Next, Request, Response}};

/// {name} middleware
pub struct {struct_name};

#[async_trait]
impl Middleware for {struct_name} {{
    async fn handle(&self, request: Request, next: Next) -> Response {{
        // TODO: Implement middleware logic
        next(request).await
    }}
}}
"#,
        name = name,
        struct_name = struct_name
    )
}

/// Template for generating new controller with make:controller command
pub fn controller_template(name: &str) -> String {
    format!(
        r#"//! {name} controller

use kit::{{handler, json_response, Request, Response}};

#[handler]
pub async fn invoke(_req: Request) -> Response {{
    json_response!({{
        "controller": "{name}"
    }})
}}
"#,
        name = name
    )
}

/// Template for generating new action with make:action command
pub fn action_template(name: &str, struct_name: &str) -> String {
    format!(
        r#"//! {name} action

use kit::injectable;

#[injectable]
pub struct {struct_name} {{
    // Dependencies injected via container
}}

impl {struct_name} {{
    pub fn execute(&self) {{
        // TODO: Implement action logic
    }}
}}
"#,
        name = name,
        struct_name = struct_name
    )
}

/// Template for generating new Inertia page with make:inertia command
pub fn inertia_page_template(component_name: &str) -> String {
    format!(
        r#"export default function {component_name}() {{
  return (
    <div className="font-sans p-8 max-w-xl mx-auto">
      <h1 className="text-3xl font-bold">{component_name}</h1>
      <p className="mt-2">
        Edit <code className="bg-gray-100 px-1 rounded">frontend/src/pages/{component_name}.tsx</code> to get started.
      </p>
    </div>
  )
}}
"#,
        component_name = component_name
    )
}

/// Template for generating new error with make:error command
pub fn error_template(struct_name: &str) -> String {
    // Convert PascalCase to human readable message
    let mut message = String::new();
    for (i, c) in struct_name.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            message.push(' ');
            message.push(c.to_lowercase().next().unwrap());
        } else if i == 0 {
            message.push(c);
        } else {
            message.push(c);
        }
    }

    format!(
        r#"//! {struct_name} error

use kit::domain_error;

#[domain_error(status = 500, message = "{message}")]
pub struct {struct_name};
"#,
        struct_name = struct_name,
        message = message
    )
}

/// Template for models/mod.rs
pub fn models_mod() -> &'static str {
    include_str!("files/backend/models/mod.rs.tpl")
}

// Actions templates

pub fn actions_mod() -> &'static str {
    include_str!("files/backend/actions/mod.rs.tpl")
}

pub fn example_action() -> &'static str {
    include_str!("files/backend/actions/example_action.rs.tpl")
}

// Config templates

pub fn config_mod() -> &'static str {
    include_str!("files/backend/config/mod.rs.tpl")
}

pub fn config_database() -> &'static str {
    include_str!("files/backend/config/database.rs.tpl")
}

pub fn config_mail() -> &'static str {
    include_str!("files/backend/config/mail.rs.tpl")
}

pub fn bootstrap() -> &'static str {
    include_str!("files/backend/bootstrap.rs.tpl")
}

// Migrations templates

pub fn migrations_mod() -> &'static str {
    include_str!("files/backend/migrations/mod.rs.tpl")
}

pub fn migrate_bin() -> &'static str {
    include_str!("files/backend/bin/migrate.rs.tpl")
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

pub fn env(project_name: &str) -> String {
    include_str!("files/root/env.tpl").replace("{project_name}", project_name)
}

pub fn env_example() -> &'static str {
    include_str!("files/root/env.example.tpl")
}

// Entity generation templates for db:sync command

/// Generate auto-generated entity file (regenerated on every sync)
pub fn entity_template(table_name: &str, columns: &[ColumnInfo]) -> String {
    let _struct_name = to_pascal_case(&singularize(table_name));

    // Generate column fields
    let column_fields: Vec<String> = columns
        .iter()
        .map(|col| {
            let rust_type = sql_type_to_rust_type(col);
            let mut attrs = Vec::new();

            if col.is_primary_key {
                attrs.push("    #[sea_orm(primary_key)]".to_string());
            }

            let field = format!("    pub {}: {},", col.name, rust_type);
            if attrs.is_empty() {
                field
            } else {
                format!("{}\n{}", attrs.join("\n"), field)
            }
        })
        .collect();

    // Find primary key columns (reserved for future use)
    let _pk_columns: Vec<&ColumnInfo> = columns.iter().filter(|c| c.is_primary_key).collect();

    format!(
        r#"// AUTO-GENERATED FILE - DO NOT EDIT
// Generated by `kit db:sync` - Changes will be overwritten
// Add custom code to src/models/{table_name}.rs instead

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "{table_name}")]
pub struct Model {{
{columns}
}}

// Note: Relation enum is required here for DeriveEntityModel macro.
// Define your actual relations in src/models/{table_name}.rs using the Related trait.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {{}}
"#,
        table_name = table_name,
        columns = column_fields.join("\n"),
    )
}

/// Generate user model file (created only once, never overwritten)
pub fn user_model_template(table_name: &str, struct_name: &str) -> String {
    format!(
        r#"//! {struct_name} model
//!
//! This file contains custom implementations for the {struct_name} model.
//! The base entity is auto-generated in src/models/entities/{table_name}.rs
//!
//! This file is NEVER overwritten by `kit db:sync` - your custom code is safe here.

// Re-export the auto-generated entity
pub use super::entities::{table_name}::*;

use sea_orm::entity::prelude::*;

// ============================================================================
// ENTITY CONFIGURATION
// Customize model behavior on insert/update/delete.
// Override methods like before_save, after_save, before_delete, etc.
// ============================================================================

impl ActiveModelBehavior for ActiveModel {{}}

// Implement Kit's Model traits for convenient querying
impl kit::database::Model for Entity {{}}
impl kit::database::ModelMut for Entity {{}}

// ============================================================================
// READ OPERATIONS
// Add custom query methods for reading/fetching data from the database.
// These methods typically return Model or Vec<Model>.
// ============================================================================

impl Model {{
    // Example: Find by a specific field
    // pub async fn find_by_email(db: &DatabaseConnection, email: &str) -> Result<Option<Self>, DbErr> {{
    //     Entity::find()
    //         .filter(Column::Email.eq(email))
    //         .one(db)
    //         .await
    // }}

    // Example: Find all with a condition
    // pub async fn find_active(db: &DatabaseConnection) -> Result<Vec<Self>, DbErr> {{
    //     Entity::find()
    //         .filter(Column::IsActive.eq(true))
    //         .all(db)
    //         .await
    // }}

    // Example: Custom computed property
    // pub fn display_name(&self) -> String {{
    //     format!("{{}} {{}}", self.first_name, self.last_name)
    // }}
}}

// ============================================================================
// WRITE OPERATIONS
// Add custom methods for creating, updating, and deleting records.
// These methods work with ActiveModel for database mutations.
// ============================================================================

impl ActiveModel {{
    // Example: Create a new record with defaults
    // pub fn new_with_defaults(name: String) -> Self {{
    //     Self {{
    //         name: Set(name),
    //         created_at: Set(chrono::Utc::now()),
    //         updated_at: Set(chrono::Utc::now()),
    //         ..Default::default()
    //     }}
    // }}

    // Example: Update timestamps before save
    // pub fn touch(&mut self) {{
    //     self.updated_at = Set(chrono::Utc::now());
    // }}
}}

// ============================================================================
// RELATIONS
// Define relationships to other entities here.
//
// Note: The Relation enum is in the auto-generated entities file and will be
// overwritten by `kit db:sync`. Define your relations using the patterns below.
// ============================================================================

// ----------------------------------------------------------------------------
// DEFINING RELATIONS
// Use Entity's relation builder methods to define RelationDef manually.
// This approach doesn't require modifying the auto-generated Relation enum.
// ----------------------------------------------------------------------------

// Example: One-to-Many relation (e.g., User has many Posts)
// impl Entity {{
//     pub fn has_many_posts() -> RelationDef {{
//         Entity::has_many(super::posts::Entity).into()
//     }}
// }}

// Example: Many-to-One / Belongs-To relation (e.g., Post belongs to User)
// impl Entity {{
//     pub fn belongs_to_user() -> RelationDef {{
//         Entity::belongs_to(super::users::Entity)
//             .from(Column::UserId)
//             .to(super::users::Column::Id)
//             .into()
//     }}
// }}

// Example: Many-to-Many through a junction table (e.g., Post has many Tags)
// impl Entity {{
//     pub fn has_many_tags() -> RelationDef {{
//         Entity::has_many(super::post_tags::Entity).into()
//     }}
// }}

// ----------------------------------------------------------------------------
// IMPLEMENTING Related<T>
// Implement the Related trait to enable .find_related() queries.
// ----------------------------------------------------------------------------

// impl Related<super::posts::Entity> for Entity {{
//     fn to() -> RelationDef {{
//         Self::has_many_posts()
//     }}
// }}

// impl Related<super::users::Entity> for Entity {{
//     fn to() -> RelationDef {{
//         Self::belongs_to_user()
//     }}
// }}

// ----------------------------------------------------------------------------
// USAGE EXAMPLE
// Once relations are defined, you can use them in queries:
//
// let user_with_posts = users::Entity::find_by_id(1)
//     .find_with_related(posts::Entity)
//     .all(db)
//     .await?;
// ----------------------------------------------------------------------------
"#,
        table_name = table_name,
        struct_name = struct_name,
    )
}

/// Generate entities/mod.rs (regenerated on every sync)
pub fn entities_mod_template(tables: &[TableInfo]) -> String {
    let mut content =
        String::from("// AUTO-GENERATED FILE - DO NOT EDIT\n// Generated by `kit db:sync`\n\n");

    for table in tables {
        content.push_str(&format!("pub mod {};\n", table.name));
    }

    content
}

// Helper functions for entity generation

fn sql_type_to_rust_type(col: &ColumnInfo) -> String {
    let col_type_upper = col.col_type.to_uppercase();
    let base_type = if col_type_upper.contains("INT") {
        if col_type_upper.contains("BIGINT") || col_type_upper.contains("INT8") {
            "i64"
        } else if col_type_upper.contains("SMALLINT") || col_type_upper.contains("INT2") {
            "i16"
        } else {
            "i32"
        }
    } else if col_type_upper.contains("TEXT")
        || col_type_upper.contains("VARCHAR")
        || col_type_upper.contains("CHAR")
        || col_type_upper.contains("CHARACTER")
    {
        "String"
    } else if col_type_upper.contains("BOOL") {
        "bool"
    } else if col_type_upper.contains("REAL") || col_type_upper.contains("FLOAT4") {
        "f32"
    } else if col_type_upper.contains("DOUBLE") || col_type_upper.contains("FLOAT8") {
        "f64"
    } else if col_type_upper.contains("TIMESTAMP") || col_type_upper.contains("DATETIME") {
        "DateTimeUtc"
    } else if col_type_upper.contains("DATE") {
        "Date"
    } else if col_type_upper.contains("TIME") {
        "Time"
    } else if col_type_upper.contains("UUID") {
        "Uuid"
    } else if col_type_upper.contains("JSON") {
        "Json"
    } else if col_type_upper.contains("BYTEA") || col_type_upper.contains("BLOB") {
        "Vec<u8>"
    } else if col_type_upper.contains("DECIMAL") || col_type_upper.contains("NUMERIC") {
        "Decimal"
    } else {
        "String" // fallback
    };

    if col.is_nullable {
        format!("Option<{}>", base_type)
    } else {
        base_type.to_string()
    }
}

fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '_' || c == '-' || c == ' ' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}

fn singularize(word: &str) -> String {
    if word.ends_with("ies") {
        format!("{}y", &word[..word.len() - 3])
    } else if word.ends_with("es") && !word.ends_with("ses") && !word.ends_with("xes") {
        word[..word.len() - 2].to_string()
    } else if word.ends_with("s") && !word.ends_with("ss") && !word.ends_with("us") {
        word[..word.len() - 1].to_string()
    } else {
        word.to_string()
    }
}
