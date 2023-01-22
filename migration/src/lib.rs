pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_profile;
mod m20230114_000001_alter_user_profile;
mod m20230122_000001_create_subscribe_source;
mod m20230122_000002_create_user_subscribe_source;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user_profile::Migration),
            Box::new(m20230114_000001_alter_user_profile::Migration),
            Box::new(m20230122_000001_create_subscribe_source::Migration),
            Box::new(m20230122_000002_create_user_subscribe_source::Migration),
        ]
    }
}
