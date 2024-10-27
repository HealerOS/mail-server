use crate::config::system_config::get_config;
use crate::exception::biz_exception::BizResult;
use migration::Migrator;
use sea_orm_migration::MigratorTrait;
use secrecy::ExposeSecret;

pub async fn migrate_db_up() -> BizResult<()> {
    let system_config = get_config()?;
    let connection =
        sea_orm::Database::connect(system_config.db_settings.connection_url().expose_secret())
            .await
            .expect("Fail to connect DB");
    Migrator::up(&connection, None)
        .await
        .expect("Fail to migrate DB up");
    Ok(())
}

pub async fn migrate_db_down() -> BizResult<()> {
    let system_config = get_config()?;
    let connection =
        sea_orm::Database::connect(system_config.db_settings.connection_url().expose_secret())
            .await
            .expect("Fail to connect DB");
    Migrator::down(&connection, None)
        .await
        .expect("Fail to migrate DB down");
    Ok(())
}
