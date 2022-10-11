use crate::app::user::model::*;
use common::utils::get_current_filename;
use sea_orm::sea_query::Table;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        // "m20220101_000001_create_table"
        get_current_filename()
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .primary_key()
                            .not_null()
                            .uuid()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Column::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Column::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Column::DeletedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Column::Age).tiny_integer())
                    .col(ColumnDef::new(Column::Email).not_null())
                    .col(
                        ColumnDef::new(Column::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Column::FirstName).string())
                    .col(ColumnDef::new(Column::LastName).string())
                    .col(ColumnDef::new(Column::Disabled).boolean())
                    .col(ColumnDef::new(Column::LastLogin).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entity).to_owned())
            .await
    }
}
