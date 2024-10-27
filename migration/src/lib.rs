pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20241027_140317_add_status;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        // 有新的要加在后面
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20241027_140317_add_status::Migration),
        ]
    }
}
