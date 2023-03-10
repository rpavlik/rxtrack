// Copyright 2022-2023, Ryan Pavlik <ryan@ryanpavlik.com>
// SPDX-License-Identifier: GPL3+

/// Fill request constructed from discrete events.
use crate::{entities::{prelude::*, events}, fill_request::FillRequest, Error, FillRequestId, RxId};
use migration::EventType;
use sea_orm::{
    prelude::TimeDate, ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait,
    EntityTrait, Order, QueryFilter, QueryOrder, TryIntoModel, Iterable,
};
use time::Date;

pub struct ImplicitFillRequest {
    id: FillRequestId,
    date_requested: Option<Date>,
    date_filled: Option<Date>,
    date_picked_up: Option<Date>,
    closed: bool,
}

impl FillRequest for ImplicitFillRequest {
    fn fill_request_id(&self) -> FillRequestId {
        self.id.into()
    }

    fn date_requested(&self) -> &Option<Date> {
        &self.date_requested
    }

    fn date_filled(&self) -> &Option<Date> {
        &self.date_filled
    }

    fn date_picked_up(&self) -> &Option<Date> {
        &self.date_picked_up
    }

    fn closed(&self) -> bool {
        self.closed
    }
}

/// Find an existing open fill request for a given rx, if any.
async fn find_existing_open_fill_request(
    db: &impl ConnectionTrait,
    rx: RxId,
) -> Result<Option<ImplicitFillRequest>, Error> {
    let x = EventType::iter().map(|event_type| Events::find().filter(events::Column::RxId.eq(i32::from(rx))).order_by_desc(col), ord))
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
        ConnectionTrait, Database, DatabaseBackend, MockDatabase, Transaction, Value::Bool,
        Value::*,
    };
    use time::{Date, Month};

    use crate::{
        entities::fill_request, fill_request::FillRequest, rx::add_rx, Error, FillRequestId, RxId,
    };

    use super::{find_existing_open_fill_request, record_fill_request};

    // async fn setup_schema(db: &impl ConnectionTrait) -> Result<(), Error> {
    //     let schema = Schema::new(DatabaseBackend::Sqlite);
    //     // let stmt = ;
    //     let stmt = schema.create_table_from_entity(rx_info::Entity);
    //     db.execute(db.get_database_backend().build(&stmt)).await?;

    //     let stmt = schema.create_table_from_entity(fill_request::Entity);
    //     db.execute(db.get_database_backend().build(&stmt)).await?;
    //     Ok(())
    // }

    struct Fixture<D> {
        db: D,
        amox_id: RxId,
        pred_id: RxId,
    }

    async fn make_inmemory_db() -> Result<Fixture<impl ConnectionTrait>, Error> {
        let db = Database::connect("sqlite::memory:").await?;
        Migrator::up(&db, None).await?;
        let amox_id = add_rx(&db, "amoxicillin").await?;
        let pred_id = add_rx(&db, "prednisone").await?;
        Ok(Fixture {
            db,
            amox_id,
            pred_id,
        })
    }

    #[async_std::test]
    async fn test_record_fill_request() -> Result<(), Error> {
        let date = Date::from_calendar_date(2023, Month::January, 1).unwrap();
        let Fixture {
            db,
            amox_id,
            pred_id,
        } = make_inmemory_db().await?;

        let existing = find_existing_open_fill_request(&db, amox_id).await?;
        assert!(existing.is_none());

        let request_id = record_fill_request(&db, amox_id, date).await?;

        let existing = find_existing_open_fill_request(&db, amox_id).await?;
        assert_eq!(existing.map(FillRequestId::from), Some(request_id));

        let request_id_2 = record_fill_request(&db, amox_id, date.next_day().unwrap()).await?;

        let existing_2 = find_existing_open_fill_request(&db, amox_id)
            .await?
            .unwrap();
        assert_eq!(existing_2.fill_request_id(), request_id_2);
        assert_eq!(*existing_2.date_requested(), date.next_day());
        assert!(existing_2.date_filled().is_none());
        assert!(existing_2.date_picked_up().is_none());
        Ok(())
    }

    #[async_std::test]
    async fn test_record_second_fill_request() -> Result<(), Error> {
        let date = Date::from_calendar_date(2023, Month::January, 1).unwrap();
        let Fixture {
            db,
            amox_id,
            pred_id,
        } = make_inmemory_db().await?;

        let request_id = record_fill_request(&db, amox_id, date).await?;

        let request_id_2 = record_fill_request(&db, amox_id, date.next_day().unwrap()).await?;
        assert_ne!(request_id, request_id_2);

        let existing_2 = find_existing_open_fill_request(&db, amox_id)
            .await?
            .unwrap();
        assert_eq!(existing_2.fill_request_id(), request_id_2);
        assert_eq!(*existing_2.date_requested(), date.next_day());
        assert!(existing_2.date_filled().is_none());
        assert!(existing_2.date_picked_up().is_none());
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
                    date_requested: Some(date),
                    date_filled: None,
                    date_picked_up: None,
                    closed: false,
                }],
            ])
            .into_connection();
        let result = record_fill_request(&db, RxId(5), date).await?;
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
                    vec![Int(Some(5)), TimeDate(Some(Box::new(date)))]
                )
            ]
        );

        Ok(())
    }
}
