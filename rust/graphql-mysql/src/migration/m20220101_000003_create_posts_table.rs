use crate::app::post::model::*;
use crate::app::user::model as user;
use common::utils::get_current_filename;
use sea_orm::sea_query::Table;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
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
                    .col(ColumnDef::new(Column::UserId).not_null())
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
                    .col(ColumnDef::new(Column::Title).text())
                    .col(ColumnDef::new(Column::Content).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_posts_users")
                            .from(self::Entity, Column::UserId)
                            .to(user::Entity, user::Column::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
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
