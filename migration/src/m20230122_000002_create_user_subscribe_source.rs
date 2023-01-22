use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

use super::m20220101_000001_create_user_profile::UserProfile;
use super::m20230122_000001_create_subscribe_source::SubscribeSource;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(UserSubscribeSource::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(UserSubscribeSource::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(UserSubscribeSource::UserId).integer().not_null())
                .col(ColumnDef::new(UserSubscribeSource::SubscribeSourceId).integer().not_null())
                .col(ColumnDef::new(UserSubscribeSource::SubscribeSourceName).string().not_null())
                .col(ColumnDef::new(UserSubscribeSource::SubscribeSourceIcon).string().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .name("FK-UserSubscribeSource_UserId-UserProfile_Id")
                        .from(UserSubscribeSource::Table, UserSubscribeSource::UserId)
                        .to(UserProfile::Table, UserProfile::Id)
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("FK-UserSubscribeSource_SubscribeSourceId-SubscribeSource_Id")
                        .from(UserSubscribeSource::Table, UserSubscribeSource::SubscribeSourceId)
                        .to(SubscribeSource::Table, SubscribeSource::Id)
                )
                .to_owned(),
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserSubscribeSource::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum UserSubscribeSource {
    Table,
    Id,
    UserId,
    SubscribeSourceId,
    SubscribeSourceName,
    SubscribeSourceIcon
}
