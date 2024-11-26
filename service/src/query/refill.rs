//! Refill query services for the `scrounch_backend` application.
//!
//! This module defines services and functions related to querying refill data in the application.
//! It encapsulates the logic for retrieving refill information from the database based on various
//! criteria such as refill ID, name, or category. These services provide a layer of abstraction
//! over database interactions, allowing for efficient and consistent data retrieval related to
//! refills.

use crate::{
    query::Query,
    r#macro::{cache_get, cache_mget, cache_mset, cache_set},
    Connection,
};
use ::entity::models::{refill, refill::Entity as Refill};
use sea_orm::*;

impl Query {
    pub async fn find_refill_by_id(
        conn: &Connection,
        id: uuid::Uuid,
    ) -> Result<Option<refill::Model>, DbErr> {
        #[cfg(feature = "cache")]
        cache_get!(conn, format!("refill:{id}"), refill::Model);

        let result = Refill::find_by_id(id).one(&conn.db_connection).await?;

        #[cfg(feature = "cache")]
        if let Some(model) = &result {
            cache_set!(conn, format!("refill:{id}"), model, 60 * 15);
        }

        Ok(result)
    }

    pub async fn list_refills_with_condition<
        F: sea_query::IntoCondition + std::fmt::Debug + Clone,
        S: IntoIterator<Item = (impl IntoSimpleExpr, Order)> + std::fmt::Debug + Clone,
        A: Into<u64> + Copy,
        P: Into<u64> + Copy,
    >(
        conn: &Connection,
        filter: F,
        sort: S,
        page: A,
        per_page: P,
    ) -> Result<Vec<refill::Model>, DbErr> {
        #[cfg(feature = "cache")]
        cache_mget!(
            conn,
            format!(
                "refills:{filter:?}-{sort:?}-{}/{}",
                page.into(),
                per_page.into()
            ),
            refill::Model
        );

        let mut query = Refill::find().filter(filter.clone());
        for (column, order) in sort.clone() {
            query = query.order_by_with_nulls(column, order, sea_query::NullOrdering::Last);
        }
        let query = query.paginate(&conn.db_connection, per_page.into());

        let result = query.fetch_page(page.into()).await?;

        #[cfg(feature = "cache")]
        cache_mset!(
            conn,
            format!(
                "refills:{filter:?}-{sort:?}-{}/{}",
                page.into(),
                per_page.into()
            ),
            result,
            60 * 15,
            "refill:"
        );

        Ok(result)
    }

    pub async fn count_refills_with_condition<F: sea_query::IntoCondition>(
        conn: &Connection,
        filter: F,
    ) -> Result<u64, DbErr> {
        Refill::find()
            .filter(filter)
            .count(&conn.db_connection)
            .await
    }
}
