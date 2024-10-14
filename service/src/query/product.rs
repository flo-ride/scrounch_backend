use crate::{
    query::Query,
    r#macro::{cache_get, cache_mget, cache_mset, cache_set},
    Connection,
};
use ::entity::{product, product::Entity as Product};
use sea_orm::*;

impl Query {
    pub async fn find_product_by_id(
        conn: &Connection,
        id: uuid::Uuid,
    ) -> Result<Option<product::Model>, DbErr> {
        #[cfg(feature = "cache")]
        cache_get!(conn, format!("product:{id}"), product::Model);

        let result = Product::find_by_id(id).one(&conn.db_connection).await?;

        #[cfg(feature = "cache")]
        if let Some(model) = &result {
            cache_set!(conn, format!("product:{id}"), model, 60 * 60 * 3);
        }

        Ok(result)
    }

    pub async fn list_products_with_condition<
        F: sea_query::IntoCondition + std::fmt::Debug + Clone,
        A: Into<u64> + Copy,
        P: Into<u64> + Copy,
    >(
        conn: &Connection,
        filter: F,
        page: A,
        per_page: P,
    ) -> Result<Vec<product::Model>, DbErr> {
        #[cfg(feature = "cache")]
        cache_mget!(
            conn,
            format!("products:{filter:?}-{}/{}", page.into(), per_page.into()),
            product::Model
        );

        let result = Product::find()
            .filter(filter.clone())
            .paginate(&conn.db_connection, per_page.into())
            .fetch_page(page.into())
            .await?;

        #[cfg(feature = "cache")]
        cache_mset!(
            conn,
            format!("products:{filter:?}-{}/{}", page.into(), per_page.into()),
            result,
            60 * 60 * 3,
            "product:"
        );

        Ok(result)
    }

    pub async fn count_products_with_condition<F: sea_query::IntoCondition>(
        conn: &Connection,
        filter: F,
    ) -> Result<u64, DbErr> {
        Product::find()
            .filter(filter)
            .count(&conn.db_connection)
            .await
    }
}
