// Copyright 2022-2023, Ryan Pavlik <ryan@ryanpavlik.com>
// SPDX-License-Identifier: GPL3+

//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "rx_info")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub rx_id: i32,
    pub rx_name: String,
    pub hidden: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::fill_request::Entity")]
    FillRequest,
    #[sea_orm(has_many = "super::reminder_policy::Entity")]
    ReminderPolicy,
}

impl Related<super::fill_request::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FillRequest.def()
    }
}

impl Related<super::reminder_policy::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReminderPolicy.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
