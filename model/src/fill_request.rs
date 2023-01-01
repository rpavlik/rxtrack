// Copyright 2022, Ryan Pavlik <ryan@ryanpavlik.com>
// SPDX-License-Identifier: GPL3+

use sea_orm::{
    prelude::TimeDate, sea_query::OnConflict, ActiveModelTrait, ActiveValue::Set, ColumnTrait,
    ConnectionTrait, EntityTrait, Order, QueryFilter, QueryOrder, TryIntoModel,
};

use crate::{
    entities::{fill_request, rx_info},
    Error, FillRequestId, RxId,
};

pub enum RxAddOutcome {
    AlreadyExists(RxId),
    Created(RxId),
}

/// Find an existing open fill request for a given rx, if any.
async fn find_existing_open_fill_request(
    db: &impl ConnectionTrait,
    rx: RxId,
) -> Result<Option<fill_request::Model>, Error> {
    let request = fill_request::Entity::find()
        .filter(fill_request::Column::Closed.eq(false))
        .filter(fill_request::Column::RxId.eq(rx.0))
        .order_by(fill_request::Column::DateRequested, Order::Desc)
        .one(db)
        .await?;
    Ok(request)
}

/// Create a new fill request for an rx, closing any previous open one (if any).
/// Returns the fill request ID.
pub async fn record_fill_request(
    db: &impl ConnectionTrait,
    rx: RxId,
    request_date: TimeDate,
) -> Result<FillRequestId, Error> {
    // Close any existing request for this prescription
    let existing_request = find_existing_open_fill_request(db, rx).await?;
    if let Some(request) = existing_request {
        let mut request: fill_request::ActiveModel = request.into();
        request.closed = Set(true);
        request.save(db).await?;
    }

    let request = fill_request::ActiveModel {
        rx_id: Set(rx.0),
        date_requested: Set(Some(request_date)),
        ..Default::default()
    };

    let res = fill_request::Entity::insert(request).exec(db).await?;
    Ok(FillRequestId(res.last_insert_id))
}

/// Records the fill and pick-up of an rx. If there is an open fill request, it is updated and closed.
/// Otherwise a new fill request is created and closed.
/// Returns the fill request ID.
pub async fn record_pickup(
    db: &impl ConnectionTrait,
    rx: RxId,
    fill_date: TimeDate,
    pickup_date: TimeDate,
) -> Result<FillRequestId, Error> {
    let existing_request = find_existing_open_fill_request(db, rx).await?;

    let mut request: fill_request::ActiveModel = match existing_request {
        // Closing an existing request
        Some(request) => request.into(),
        // Making and closing a new request
        None => fill_request::ActiveModel {
            rx_id: Set(rx.0),
            ..Default::default()
        },
    };

    request.date_filled = Set(Some(fill_date));
    request.date_picked_up = Set(Some(pickup_date));
    request.closed = Set(true);

    let request: fill_request::Model = request.save(db).await?.try_into_model()?;

    Ok(FillRequestId(request.id))
}

#[cfg(test)]
mod test {

    use migration::{Migrator, MigratorTrait};
    use sea_orm::{
        entity::prelude::*, entity::*, ConnectionTrait, Database, DatabaseBackend, MockDatabase,
        Schema,
    };

    use crate::{
        entities::{fill_request, rx_info},
        Error,
    };

    async fn setup_schema(db: &impl ConnectionTrait) -> Result<(), Error> {
        let schema = Schema::new(DatabaseBackend::Sqlite);
        // let stmt = ;
        let stmt = schema.create_table_from_entity(rx_info::Entity);
        db.execute(db.get_database_backend().build(&stmt)).await?;

        let stmt = schema.create_table_from_entity(fill_request::Entity);
        db.execute(db.get_database_backend().build(&stmt)).await?;
        Ok(())
    }

    async fn make_inmemory_db() -> Result<impl ConnectionTrait, Error> {
        let db = Database::connect("sqlite::memory:").await?;
        // Create MockDatabase with mock query results
        Migrator::up(&db, None).await?;
        Ok(db)
    }

    #[async_std::test]
    async fn test_add_rx() -> Result<(), Error> {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![
                // First query result
                vec![rx_info::Model {
                    rx_id: 1,
                    rx_name: "amoxicillin".to_owned(),
                    hidden: false,
                }],
            ])
            .into_connection();

        // // Find a cake from MockDatabase
        // // Return the first query result
        // assert_eq!(
        //     rx_info::Entity::find().one(&db).await?,
        //     Some(cake::Model {
        //         id: 1,
        //         name: "New York Cheese".to_owned(),
        //     })
        // );

        // // Find all cakes from MockDatabase
        // // Return the second query result
        // assert_eq!(
        //     cake::Entity::find().all(&db).await?,
        //     vec![
        //         cake::Model {
        //             id: 1,
        //             name: "New York Cheese".to_owned(),
        //         },
        //         cake::Model {
        //             id: 2,
        //             name: "Chocolate Forest".to_owned(),
        //         },
        //     ]
        // );

        // // Find all cakes with its related fruits
        // assert_eq!(
        //     cake::Entity::find()
        //         .find_also_related(fruit::Entity)
        //         .all(&db)
        //         .await?,
        //     vec![(
        //         cake::Model {
        //             id: 1,
        //             name: "Apple Cake".to_owned(),
        //         },
        //         Some(fruit::Model {
        //             id: 2,
        //             name: "Apple".to_owned(),
        //             cake_id: Some(1),
        //         })
        //     )]
        // );

        // // Checking transaction log
        // assert_eq!(
        //     db.into_transaction_log(),
        //     vec![
        //         Transaction::from_sql_and_values(
        //             DatabaseBackend::Postgres,
        //             r#"SELECT "cake"."id", "cake"."name" FROM "cake" LIMIT $1"#,
        //             vec![1u64.into()]
        //         ),
        //         Transaction::from_sql_and_values(
        //             DatabaseBackend::Postgres,
        //             r#"SELECT "cake"."id", "cake"."name" FROM "cake""#,
        //             vec![]
        //         ),
        //         Transaction::from_sql_and_values(
        //             DatabaseBackend::Postgres,
        //             r#"SELECT "cake"."id" AS "A_id", "cake"."name" AS "A_name", "fruit"."id" AS "B_id", "fruit"."name" AS "B_name", "fruit"."cake_id" AS "B_cake_id" FROM "cake" LEFT JOIN "fruit" ON "cake"."id" = "fruit"."cake_id""#,
        //             vec![]
        //         ),
        //     ]
        // );

        Ok(())
    }
}
