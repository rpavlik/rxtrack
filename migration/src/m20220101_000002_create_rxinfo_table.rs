use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_person_table::Person;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub enum RxInfo {
    Table,
    RxId,
    PersonId,
    RxName,
}

#[derive(Iden)]
pub enum FillRequest {
    Table,
    Id,
    RxId,
    DateRequested,
    DateFilled,
    DatePickedUp,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RxInfo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RxInfo::RxId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RxInfo::PersonId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-rx-person_id")
                            .from(RxInfo::Table, RxInfo::PersonId)
                            .to(Person::Table, Person::PersonId),
                    )
                    .col(ColumnDef::new(RxInfo::RxName).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(FillRequest::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FillRequest::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(FillRequest::RxId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-fill-rx_id")
                            .from(FillRequest::Table, FillRequest::Id)
                            .to(RxInfo::Table, RxInfo::RxId),
                    )
                    .col(ColumnDef::new(FillRequest::DateRequested).date().not_null())
                    .col(ColumnDef::new(FillRequest::DateFilled).date())
                    .col(ColumnDef::new(FillRequest::DatePickedUp).date())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FillRequest::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RxInfo::Table).to_owned())
            .await
    }
}
