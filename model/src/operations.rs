// Copyright 2022, Ryan Pavlik <ryan@ryanpavlik.com>
// SPDX-License-Identifier: GPL3+

use sea_orm::prelude::TimeDate;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{ConnectionTrait, DbErr, EntityTrait};

use crate::entities::fill_request;
use crate::{entities::rx_info, RxId};
use crate::{Error, FillRequestId};

/// Add a new prescription, receiving the ID.
pub async fn add_rx(name: &str, db: &impl ConnectionTrait) -> Result<RxId, Error> {
    if name.is_empty() {
        return Err(Error::EmptyRxName);
    }
    let rx = rx_info::ActiveModel {
        rx_name: Set(name.to_owned()),
        ..Default::default()
    };
    let res = rx_info::Entity::insert(rx).exec(db).await?;
    Ok(RxId(res.last_insert_id))
}

pub async fn record_fill_request(
    rx: RxId,
    request_date: TimeDate,
    db: &impl ConnectionTrait,
) -> Result<FillRequestId, Error> {
    let request = fill_request::ActiveModel {
        rx_id: Set(rx.0),
        date_requested: Set(Some(request_date)),
        ..Default::default()
    };

    let res = fill_request::Entity::insert(request).exec(db).await?;
    Ok(FillRequestId(res.last_insert_id))
}

pub async fn record_pickup(
    rx: RxId,
    fill_date: TimeDate,
    pickup_date: TimeDate,
    db: &impl ConnectionTrait,
) -> Result<FillRequestId, Error> {
    
    // We may not have recorded 
    let request = fill_request::ActiveModel {
        rx_id: Set(rx.0),
        date_filled: Set(Some(fill_date)),
        date_picked_up: Set(Some(pickup_date)),
        ..Default::default()
    };
}
