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

/// Add a new prescription, receiving the ID.
pub async fn add_rx(db: &impl ConnectionTrait, name: &str) -> Result<RxId, Error> {
    let name = name.trim();
    if name.is_empty() {
        return Err(Error::EmptyRxName);
    }
    let rx = rx_info::ActiveModel {
        rx_name: Set(name.to_owned()),
        ..Default::default()
    };
    let res = rx_info::Entity::insert(rx)
        // .on_conflict(
        //     OnConflict::column(rx_info::Column::RxName)
        //         .do_nothing()
        //         .to_owned(),
        // )
        .exec(db)
        .await?;
    Ok(RxId(res.last_insert_id))
}

#[cfg(test)]
mod test {

    use async_std::io::empty;
    use migration::{Migrator, MigratorTrait};
    use sea_orm::{
        entity::prelude::*, entity::*, ConnectionTrait, Database, DatabaseBackend, MockDatabase,
        MockExecResult, Schema, Transaction,
    };

    use super::*;
    use crate::Error;

    // async fn setup_schema(db: &impl ConnectionTrait) -> Result<(), Error> {
    //     let schema = Schema::new(DatabaseBackend::Sqlite);
    //     // let stmt = ;
    //     let stmt = schema.create_table_from_entity(rx_info::Entity);
    //     db.execute(db.get_database_backend().build(&stmt)).await?;

    //     let stmt = schema.create_table_from_entity(fill_request::Entity);
    //     db.execute(db.get_database_backend().build(&stmt)).await?;
    //     Ok(())
    // }

    // async fn make_inmemory_db() -> Result<impl ConnectionTrait, Error> {
    //     let db = Database::connect("sqlite::memory:").await?;
    //     // Create MockDatabase with mock query results
    //     Migrator::up(&db, None).await?;
    //     Ok(db)
    // }

    #[async_std::test]
    async fn test_add_rx() -> Result<(), Error> {
        // Check input verification
        let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
        assert_eq!(add_rx(&db, "").await, Err(Error::EmptyRxName));
        assert_eq!(add_rx(&db, "  ").await, Err(Error::EmptyRxName));
        assert_eq!(db.into_transaction_log(), vec![]);
        let make_empty_results = || {
            let empty_results: Vec<Vec<rx_info::Model>> = vec![vec![]];
            empty_results
        };
        // Check normal operation
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(make_empty_results())
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 5,
                rows_affected: 1,
            }])
            .into_connection();
        assert_eq!(add_rx(&db, "amoxicillin").await, Ok(RxId(5)));
        assert_eq!(
            db.into_transaction_log(),
            vec![Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"INSERT INTO "rx_info" ("rx_name") VALUES ($1) RETURNING "id", "name""#,
                vec!["amoxicillin".into()]
            )]
        );

        // check trimming
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(make_empty_results())
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 5,
                rows_affected: 1,
            }])
            .into_connection();
        assert_eq!(add_rx(&db, "  amoxicillin  ").await, Ok(RxId(5)));
        assert_eq!(
            db.into_transaction_log(),
            vec![Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"INSERT INTO "rx_info" ("rx_name") VALUES ($1) RETURNING "id", "name""#,
                vec!["amoxicillin".into()]
            )]
        );
        Ok(())
    }
}
