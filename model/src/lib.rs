// Copyright 2022, Ryan Pavlik <ryan@ryanpavlik.com>
// SPDX-License-Identifier: GPL3+

use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, DeriveColumn, EntityTrait, EnumIter, QueryFilter,
    QuerySelect,
};

pub mod entities;

// /// Person ID
// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct PersonId(u32);

/// Prescription ID
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RxId(u32);

/// Fill Request ID
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FillRequestId(u32);

// #[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
// enum QueryRxInfoAs {
//     RxId,
//     RxName,
// }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
