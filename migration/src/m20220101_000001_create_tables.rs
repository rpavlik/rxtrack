// Copyright 2022-2023, Ryan Pavlik <ryan@ryanpavlik.com>
// SPDX-License-Identifier: GPL3+

use sea_orm_migration::prelude::*;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{ActiveEnum, Iterable},
};

use crate::EventType;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
pub enum RxInfo {
    Table,
    RxId,
    RxName,
    Hidden,
}

#[derive(Iden)]
pub enum FillRequest {
    Table,
    Id,
    RxId,
    DateRequested,
    DateFilled,
    DatePickedUp,
    Closed,
}

#[derive(Iden)]
pub enum ReminderPolicy {
    Table,
    ReminderId,
    RxId,
    /// an enum identifying what event's date we start from
    StartingDate,
    /// boolean whether we add the rx duration
    IncludeRxDuration,
    /// signed int offset
    Offset,
    /// Whether to allow reminder on a saturday or back it up
    AllowSaturday,
    /// Whether to allow reminder on a sunday or back it up
    AllowSunday,
    Description,
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
                    // TODO restore person table eventually
                    // .col(ColumnDef::new(RxInfo::PersonId).integer().not_null())
                    // .foreign_key(
                    //     ForeignKey::create()
                    //         .name("fk-rx-person_id")
                    //         .from(RxInfo::Table, RxInfo::PersonId)
                    //         .to(Person::Table, Person::PersonId),
                    // )
                    .col(ColumnDef::new(RxInfo::RxName).string().not_null())
                    .col(
                        ColumnDef::new(RxInfo::Hidden)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
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
                            .from(FillRequest::Table, FillRequest::RxId)
                            .to(RxInfo::Table, RxInfo::RxId),
                    )
                    .col(ColumnDef::new(FillRequest::DateRequested).date())
                    .col(ColumnDef::new(FillRequest::DateFilled).date())
                    .col(ColumnDef::new(FillRequest::DatePickedUp).date())
                    .col(
                        ColumnDef::new(FillRequest::Closed)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ReminderPolicy::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ReminderPolicy::ReminderId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ReminderPolicy::RxId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-reminder-rx_id")
                            .from(ReminderPolicy::Table, ReminderPolicy::RxId)
                            .to(RxInfo::Table, RxInfo::RxId),
                    )
                    .col(
                        ColumnDef::new(ReminderPolicy::StartingDate)
                            .enumeration(EventType::name(), EventType::iter()),
                    )
                    .col(
                        ColumnDef::new(ReminderPolicy::IncludeRxDuration)
                            .boolean()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ReminderPolicy::Offset).integer().not_null())
                    .col(
                        ColumnDef::new(ReminderPolicy::AllowSaturday)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ReminderPolicy::AllowSunday)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ReminderPolicy::Description)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ReminderPolicy::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(FillRequest::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RxInfo::Table).to_owned())
            .await
    }
}
