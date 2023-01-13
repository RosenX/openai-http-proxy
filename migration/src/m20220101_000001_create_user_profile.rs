use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(UserProfile::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(UserProfile::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(UserProfile::Username).string().not_null())
                .col(ColumnDef::new(UserProfile::Email).string().not_null())
                .col(ColumnDef::new(UserProfile::HashPassword).string().not_null())
                .to_owned(),
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserProfile::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum UserProfile {
    Table,
    Id,
    Username,
    Email,
    HashPassword,
}
