// Copyright 2022-2023, Ryan Pavlik <ryan@ryanpavlik.com>
// SPDX-License-Identifier: GPL3+

use sea_orm_migration::prelude::*;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{ActiveEnum, Iterable},
};

use crate::m20220101_000001_create_tables::RxInfo;
use crate::EventType;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub enum Events {
    Table,
    Id,
    RxId,
    Event,
    Date,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Events::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Events::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Events::RxId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-events-rx_id")
                            .from(Events::Table, Events::RxId)
                            .to(RxInfo::Table, RxInfo::RxId),
                    )
                    .col(
                        ColumnDef::new(Events::Event)
                            .enumeration(EventType::name(), EventType::iter())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Events::Date).date().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Events::Table).to_owned())
            .await
    }
}
