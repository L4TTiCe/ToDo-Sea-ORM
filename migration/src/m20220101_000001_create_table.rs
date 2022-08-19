use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Task::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Task::Title).string().not_null())
                    .col(ColumnDef::new(Task::Completed).boolean().not_null())
                    .col(
                        ColumnDef::new(Task::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .extra("DEFAULT NOW()".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Task::Deadline)
                            .timestamp_with_time_zone()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Task {
    Table,
    Id,
    Title,
    Completed,
    CreatedAt,
    Deadline,
}
