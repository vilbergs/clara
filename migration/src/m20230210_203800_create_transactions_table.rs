use chrono::Utc;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Transaction::Table).to_owned())
            .await
    }

    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Transaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Transaction::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Transaction::UserId).integer().not_null())
                    .col(ColumnDef::new(Transaction::CategoryId).integer())
                    .col(ColumnDef::new(Transaction::Name).string().not_null())
                    .col(ColumnDef::new(Transaction::Description).string())
                    .col(ColumnDef::new(Transaction::Amount).big_integer().not_null())
                    .col(ColumnDef::new(Transaction::Type).string().not_null())
                    .col(ColumnDef::new(Transaction::Repeat).string())
                    .col(
                        ColumnDef::new(Transaction::TransactionDate)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Transaction::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Utc::now().naive_utc().to_string()),
                    )
                    .col(
                        ColumnDef::new(Transaction::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Transaction {
    Table,
    Id,
    UserId,
    CategoryId,
    Name,
    Description,
    Amount,
    Type,
    Repeat,
    TransactionDate,
    CreatedAt,
    UpdatedAt,
}
