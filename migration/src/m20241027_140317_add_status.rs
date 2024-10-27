use crate::sea_orm::EnumIter;
use sea_orm_migration::sea_orm::{DbBackend, Iterable, Schema};
use sea_orm_migration::sea_query::extension::postgres::Type;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let status_enum = Type::create()
            .as_enum(StatusEnum)
            .values(StatusVariants::iter())
            .to_owned();

        let table = Table::alter()
            .table(Subscriptions::Table)
            .add_column_if_not_exists(
                ColumnDef::new(Subscriptions::Status)
                    .enumeration(StatusEnum, StatusVariants::iter())
                    .not_null()
                    .comment("用户的订阅状态")
                    .default("Waiting"),
            )
            .to_owned();

        manager.create_type(status_enum).await?;
        manager.alter_table(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let table = Table::alter()
            .table(Subscriptions::Table)
            .drop_column(Subscriptions::Status)
            .to_owned();

        let status_enum = Type::drop().name(StatusEnum).to_owned();

        manager.alter_table(table).await?;
        manager.drop_type(status_enum).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
struct StatusEnum;

#[derive(DeriveIden, EnumIter)]
enum StatusVariants {
    #[sea_orm(iden = "Waiting")]
    Waiting,
    #[sea_orm(iden = "Confirmed")]
    Confirmed,
}

#[derive(DeriveIden)]
enum Subscriptions {
    Table,
    Status,
}
