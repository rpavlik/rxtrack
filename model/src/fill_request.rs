// Copyright 2022, Ryan Pavlik <ryan@ryanpavlik.com>
// SPDX-License-Identifier: GPL3+

use sea_orm::{
    prelude::TimeDate, sea_query::OnConflict, ActiveModelTrait, ActiveValue::Set, ColumnTrait,
    ConnectionTrait, EntityTrait, Order, QueryFilter, QueryOrder, Select, TryIntoModel,
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
async fn select_open_fill_request(
    db: &impl ConnectionTrait,
    rx: RxId,
) -> Select<fill_request::Entity> {
    let request = fill_request::Entity::find()
        .filter(fill_request::Column::Closed.eq(false))
        .filter(fill_request::Column::RxId.eq(rx.0));
    request
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
        entity::prelude::*, ConnectionTrait, Database, DatabaseBackend, MockDatabase, Schema,
        Transaction, Value::Bool, Value::*, Values,
    };
    use time::{Date, Month};

    use crate::{
        entities::{fill_request, rx_info},
        Error, FillRequestId, RxId, rx::add_rx,
    };

    use super::record_fill_request;

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
    async fn test_integration() -> Result<(), Error> {
        
        let date = Date::from_calendar_date(2023, Month::January, 1).unwrap();
        let db = Database::connect("sqlite::memory:").await?;
        Migrator::up(&db, None).await?;
        add_rx(&db, "amoxicillin").await?;
Ok(())
    }

    #[async_std::test]
    async fn test_record_request() -> Result<(), Error> {
        // let make_empty_results = || {
        //     let empty_results: Vec<Vec<rx_info::Model>> = vec![vec![]];
        //     empty_results
        // };
        let date = Date::from_calendar_date(2023, Month::January, 1).unwrap();
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![
                vec![],
                vec![fill_request::Model {
                    id: 1,
                    rx_id: 5,
                    date_requested: Some(date.clone()),
                    date_filled: None,
                    date_picked_up: None,
                    closed: false,
                }],
            ])
            .into_connection();
        let result = record_fill_request(&db, RxId(5), date.clone()).await?;
        assert_eq!(result, FillRequestId(1));

        let log = db.into_transaction_log();
        // println!("{:?}", log);

        assert_eq!(
            log,
            vec![
                Transaction::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    r#"SELECT "fill_request"."id", "fill_request"."rx_id", "fill_request"."date_requested", "fill_request"."date_filled", "fill_request"."date_picked_up", "fill_request"."closed" FROM "fill_request" WHERE "fill_request"."closed" = $1 AND "fill_request"."rx_id" = $2 ORDER BY "fill_request"."date_requested" DESC LIMIT $3"#,
                    vec![Bool(Some(false)), Int(Some(5)), BigUnsigned(Some(1))]
                ),
                Transaction::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    r#"INSERT INTO "fill_request" ("rx_id", "date_requested") VALUES ($1, $2) RETURNING "id""#,
                    vec![Int(Some(5)), TimeDate(Some(Box::new(date.clone())))]
                )
            ]
        );

        Ok(())
    }
}
