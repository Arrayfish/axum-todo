use sea_orm_migration::{prelude::*, schema::*};
mod m20220101_000001_create_user_table;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Todo::Table)
                    .if_not_exists()
                    .col(pk_auto(Todo::Id))
                    .col(uuid(Todo::Pid))
                    .col(foreign(Todo::UserId).references(User::Table, User::Id))
                    .col(string(Todo::Done))
                    .col(string(Todo::Content))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Todo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Todo {
    Table,
    Id,
    Pid,
    UserId,
    Done,
    Content,
}
