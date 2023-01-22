// Copyright 2022-2023, Ryan Pavlik <ryan@ryanpavlik.com>
// SPDX-License-Identifier: GPL3+

pub use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{DeriveActiveEnum, EnumIter};

mod m20220101_000001_create_tables;
mod m20230122_000001_generic_event;

#[derive(Debug, PartialEq, EnumIter, DeriveActiveEnum, Iden)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum EventType {
    #[sea_orm(num_value = 0)]
    RequestFill,
    #[sea_orm(num_value = 1)]
    Fill,
    #[sea_orm(num_value = 2)]
    PickUp,
    #[sea_orm(num_value = 3)]
    RefillCancel,
}

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_tables::Migration),
            Box::new(m20230122_000001_generic_event::Migration),
        ]
    }
}
