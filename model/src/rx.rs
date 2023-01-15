// Copyright 2022, Ryan Pavlik <ryan@ryanpavlik.com>
// SPDX-License-Identifier: GPL3+

use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter,
};

use crate::{entities::rx_info, Error, RxId};

// pub enum RxAddOutcome {
//     AlreadyExists(RxId),
//     Created(RxId),
// }

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnownRx {
    pub id: RxId,
    pub name: String,
    pub hidden: bool,
}

impl From<rx_info::Model> for KnownRx {
    fn from(value: rx_info::Model) -> Self {
        KnownRx {
            id: RxId::from(value.rx_id),
            name: value.rx_name,
            hidden: value.hidden,
        }
    }
}

pub async fn list_rx(db: &impl ConnectionTrait) -> Result<Vec<KnownRx>, sea_orm::DbErr> {
    let result = rx_info::Entity::find()
        .filter(rx_info::Column::Hidden.eq(false))
        .all(db)
        .await?;
    let v: Vec<KnownRx> = result.into_iter().map(KnownRx::from).collect();
    Ok(v)
}

pub async fn list_all_rx(db: &impl ConnectionTrait) -> Result<Vec<KnownRx>, sea_orm::DbErr> {
    let result = rx_info::Entity::find().all(db).await?;
    let v: Vec<KnownRx> = result.into_iter().map(KnownRx::from).collect();
    Ok(v)
}

pub async fn get_rx(
    db: &impl ConnectionTrait,
    id: RxId,
) -> Result<Option<KnownRx>, sea_orm::DbErr> {
    let rx = rx_info::Entity::find_by_id(i32::from(id)).one(db).await?;
    Ok(rx.map(KnownRx::from))
}

#[cfg(test)]
mod test {

    use migration::{Migrator, MigratorTrait};
    use sea_orm::{Database, DatabaseBackend, MockDatabase, MockExecResult, Transaction};
    use time::{Date, Month};

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
    async fn test_operation() -> Result<(), Error> {
        let db = Database::connect("sqlite::memory:").await?;
        Migrator::up(&db, None).await?;
        assert!(list_rx(&db).await?.is_empty());
        let amox_id = add_rx(&db, "amoxicillin").await?;

        let amox_data = get_rx(&db, amox_id).await?;
        assert!(amox_data.is_some());
        let amox_data = amox_data.unwrap();
        assert_eq!(amox_data.name, "amoxicillin");
        assert_eq!(amox_data.id, amox_id);
        assert_eq!(amox_data.hidden, false);

        let pred_id = add_rx(&db, "prednisone").await?;

        let rxs = list_rx(&db).await?;
        assert_eq!(rxs.len(), 2);
        assert!(rxs.contains(&amox_data));
        assert!(rxs.iter().find(|e| e.name == "prednisone" && e.id == pred_id).is_some());
        Ok(())
    }

    #[async_std::test]
    async fn test_add_rx() -> Result<(), Error> {
        // Check input verification
        let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
        assert_eq!(add_rx(&db, "").await, Err(Error::EmptyRxName));
        assert_eq!(add_rx(&db, "  ").await, Err(Error::EmptyRxName));
        assert_eq!(db.into_transaction_log(), vec![]);
        // let make_empty_results = || {
        //     let empty_results: Vec<Vec<rx_info::Model>> = vec![vec![]];
        //     empty_results
        // };
        // Check normal operation
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            // TODO not sure why this is also needed
            .append_query_results(vec![vec![rx_info::Model {
                rx_id: 5,
                rx_name: "fake".to_owned(),
                hidden: false,
            }]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 5,
                rows_affected: 1,
            }])
            .into_connection();
        let result = add_rx(&db, "amoxicillin").await;
        assert_eq!(result, Ok(RxId(5)));
        let log = db.into_transaction_log();
        // println!("{:?}", log);
        assert_eq!(
            log,
            vec![Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"INSERT INTO "rx_info" ("rx_name") VALUES ($1) RETURNING "rx_id""#,
                vec!["amoxicillin".into()]
            )]
        );

        // check trimming
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            // TODO not sure why this is also needed
            .append_query_results(vec![vec![rx_info::Model {
                rx_id: 5,
                rx_name: "fake".to_owned(),
                hidden: false,
            }]])
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
                r#"INSERT INTO "rx_info" ("rx_name") VALUES ($1) RETURNING "rx_id""#,
                vec!["amoxicillin".into()]
            )]
        );
        Ok(())
    }
}
