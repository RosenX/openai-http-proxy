use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;



#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(SubscribeSource::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(SubscribeSource::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(SubscribeSource::UriIdentity).string().not_null().unique_key())
                .to_owned(),
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SubscribeSource::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum SubscribeSource {
    Table,
    Id,
    UriIdentity,
}