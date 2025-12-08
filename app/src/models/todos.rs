//! Todos model
//!
//! This file contains custom implementations for the Todos model.
//! The base entity is auto-generated in src/models/entities/todos.rs
//!
//! This file is NEVER overwritten by `kit db:sync` - your custom code is safe here.

// Re-export the auto-generated entity
pub use super::entities::todos::*;

use sea_orm::entity::prelude::*;

// ============================================================================
// ENTITY CONFIGURATION
// Customize model behavior on insert/update/delete.
// Override methods like before_save, after_save, before_delete, etc.
// ============================================================================

impl ActiveModelBehavior for ActiveModel {}

// Implement Kit's Model traits for convenient querying
impl kit::database::Model for Entity {}
impl kit::database::ModelMut for Entity {}

// ============================================================================
// READ OPERATIONS
// Add custom query methods for reading/fetching data from the database.
// These methods typically return Model or Vec<Model>.
// ============================================================================

impl Model {
    // Example: Find by a specific field
    // pub async fn find_by_title(db: &DatabaseConnection, title: &str) -> Result<Option<Self>, DbErr> {
    //     Entity::find()
    //         .filter(Column::Title.eq(title))
    //         .one(db)
    //         .await
    // }
}

// ============================================================================
// WRITE OPERATIONS
// Add custom methods for creating, updating, and deleting records.
// These methods work with ActiveModel for database mutations.
// ============================================================================

impl ActiveModel {
    // Example: Create a new todo with defaults
    // pub fn new_with_title(title: String) -> Self {
    //     Self {
    //         title: Set(title),
    //         ..Default::default()
    //     }
    // }
}
