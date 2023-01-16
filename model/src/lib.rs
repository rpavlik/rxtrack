// Copyright 2022-2023, Ryan Pavlik <ryan@ryanpavlik.com>
// SPDX-License-Identifier: GPL3+

use sea_orm::DbErr;

pub mod entities;
pub mod fill_request;
mod ids;
pub mod rx;

pub use ids::{FillRequestId, RxId};

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("Prescription name cannot be empty")]
    EmptyRxName,

    #[error("Database error: {0}")]
    DbError(#[from] DbErr),
}
