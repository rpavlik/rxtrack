// Copyright 2022, Ryan Pavlik <ryan@ryanpavlik.com>
// SPDX-License-Identifier: GPL3+

pub use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{DeriveActiveEnum, EnumIter};

mod m20220101_000001_create_tables;

#[derive(Debug, PartialEq, EnumIter, DeriveActiveEnum, Iden)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum EventType {
    #[sea_orm(num_value = 0)]
    Fill,
    #[sea_orm(num_value = 1)]
    PickUp,
}

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_tables::Migration)]
    }
}
