// Copyright 2022-2023, Ryan Pavlik <ryan@ryanpavlik.com>
// SPDX-License-Identifier: GPL3+

use std::fmt::Display;

use crate::entities;

/// Prescription ID
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct RxId(pub(crate) i32);

impl Display for RxId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RxId({})", self.0)
    }
}

impl From<i32> for RxId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<RxId> for i32 {
    fn from(value: RxId) -> Self {
        value.0
    }
}

impl From<entities::rx_info::Model> for RxId {
    fn from(value: entities::rx_info::Model) -> Self {
        RxId(value.rx_id)
    }
}

impl From<entities::fill_request::Model> for RxId {
    fn from(value: entities::fill_request::Model) -> Self {
        value.rx_id.into()
    }
}

/// Fill Request ID
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct FillRequestId(pub(crate) i32);

impl Display for FillRequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FillRequestId({})", self.0)
    }
}

impl From<i32> for FillRequestId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<FillRequestId> for i32 {
    fn from(value: FillRequestId) -> Self {
        value.0
    }
}

impl From<entities::fill_request::Model> for FillRequestId {
    fn from(value: entities::fill_request::Model) -> Self {
        value.id.into()
    }
}
