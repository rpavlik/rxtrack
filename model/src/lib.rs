// Copyright 2022, Ryan Pavlik <ryan@ryanpavlik.com>
// SPDX-License-Identifier: GPL3+

use sea_orm::{ConnectionTrait, DbErr, EntityTrait};

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("Prescription name cannot be empty")]
    EmptyRxName,

    #[error("Database error: {0}")]
    DbError(#[from] DbErr),
}

pub mod entities;

/// Prescription ID
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct RxId(pub(crate) i32);

/// Fill Request ID
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct FillRequestId(pub(crate) i32);

pub async fn get_prescriptions(
    db: &impl ConnectionTrait,
) -> Result<Vec<entities::rx_info::Model>, DbErr> {
    entities::rx_info::Entity::find().all(db).await
}

// pub async fn get_prescriptions_for_person(
//     person: PersonId,
//     db: &impl ConnectionTrait,
// ) -> Result<Vec<entities::rx_info::Model>, DbErr> {
//     entities::rx_info::Entity::find()
//         .filter(entities::rx_info::Column::PersonId.eq(person.0))
//         .all(db)
//         .await
// }

pub mod fill_request;
pub mod rx;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
