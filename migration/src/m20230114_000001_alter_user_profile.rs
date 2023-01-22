use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(UserProfile::Table)
                    .add_column(ColumnDef::new(UserProfile::IsPro).boolean().default(false).not_null())
                    .add_column(ColumnDef::new(UserProfile::ProEndTime).date_time())
                    .add_column(ColumnDef::new(UserProfile::CreatedTime).date_time().not_null())
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(UserProfile::Table)
                    .drop_column(UserProfile::ProEndTime)
                    .drop_column(UserProfile::IsPro)
                    .drop_column(UserProfile::CreatedTime)
                    .to_owned()
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum UserProfile {
    Table,
    IsPro,
    CreatedTime,
    ProEndTime,
}
