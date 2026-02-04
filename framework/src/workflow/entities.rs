//! SeaORM entities for workflows

pub mod workflows {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "workflows")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i64,
        pub name: String,
        pub status: String,
        #[sea_orm(column_type = "Text")]
        pub input: String,
        #[sea_orm(column_type = "Text", nullable)]
        pub output: Option<String>,
        #[sea_orm(column_type = "Text", nullable)]
        pub error: Option<String>,
        pub attempts: i32,
        pub max_attempts: i32,
        pub next_run_at: Option<chrono::NaiveDateTime>,
        pub locked_until: Option<chrono::NaiveDateTime>,
        pub worker_id: Option<String>,
        pub created_at: chrono::NaiveDateTime,
        pub updated_at: chrono::NaiveDateTime,
        pub started_at: Option<chrono::NaiveDateTime>,
        pub completed_at: Option<chrono::NaiveDateTime>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod workflow_steps {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "workflow_steps")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i64,
        pub workflow_id: i64,
        pub step_index: i32,
        pub step_name: String,
        pub status: String,
        #[sea_orm(column_type = "Text")]
        pub input: String,
        #[sea_orm(column_type = "Text", nullable)]
        pub output: Option<String>,
        #[sea_orm(column_type = "Text", nullable)]
        pub error: Option<String>,
        pub attempts: i32,
        pub created_at: chrono::NaiveDateTime,
        pub updated_at: chrono::NaiveDateTime,
        pub started_at: Option<chrono::NaiveDateTime>,
        pub completed_at: Option<chrono::NaiveDateTime>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
