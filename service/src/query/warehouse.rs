#[cfg(feature = "cache")]
use crate::r#macro::{cache_get, cache_mget, cache_mset, cache_set};
use crate::{query::Query, Connection};
use ::entity::models::{warehouse, warehouse::Entity as Warehouse};
use sea_orm::*;

impl Query {
    pub async fn find_warehouse_by_id(
        conn: &Connection,
        id: uuid::Uuid,
    ) -> Result<Option<warehouse::Model>, DbErr> {
        #[cfg(feature = "cache")]
        cache_get!(conn, format!("warehouse:{id}"), warehouse::Model);

        let result = Warehouse::find_by_id(id).one(&conn.db_connection).await?;

        #[cfg(feature = "cache")]
        if let Some(model) = &result {
            cache_set!(conn, format!("warehouse:{id}"), model, 60 * 60 * 3);
        }

        Ok(result)
    }

    pub async fn list_warehouses_with_condition<
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
    ) -> Result<Vec<warehouse::Model>, DbErr> {
        #[cfg(feature = "cache")]
        cache_mget!(
            conn,
            format!(
                "warehouses:{filter:?}-{sort:?}-{}/{}",
                page.into(),
                per_page.into()
            ),
            warehouse::Model
        );

        let mut query = Warehouse::find().filter(filter.clone());
        for (column, order) in sort.clone() {
            query = query.order_by_with_nulls(column, order, sea_query::NullOrdering::Last);
        }
        let query = query.paginate(&conn.db_connection, per_page.into());

        let result = query.fetch_page(page.into()).await?;

        #[cfg(feature = "cache")]
        cache_mset!(
            conn,
            format!(
                "warehouses:{filter:?}-{sort:?}-{}/{}",
                page.into(),
                per_page.into()
            ),
            result,
            60 * 60 * 3,
            |x: &warehouse::Model| format!("warehouse:{}", x.id)
        );

        Ok(result)
    }

    pub async fn count_warehouses_with_condition<F: sea_query::IntoCondition>(
        conn: &Connection,
        filter: F,
    ) -> Result<u64, DbErr> {
        Warehouse::find()
            .filter(filter)
            .count(&conn.db_connection)
            .await
    }
}
