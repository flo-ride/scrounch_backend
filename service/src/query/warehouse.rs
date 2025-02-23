#[cfg(feature = "cache")]
use crate::r#macro::{cache_get, cache_mget, cache_mset, cache_set};
use crate::{query::Query, Connection};
use ::entity::models::{
    prelude::{Product, Recipe, Warehouse, WarehouseProduct, WarehouseRecipe},
    product, recipe, warehouse, warehouse_product, warehouse_recipe,
};
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

    pub async fn find_warehouse_products_by_id<
        Filter: sea_query::IntoCondition + std::fmt::Debug + Clone,
        Sort: IntoIterator<Item = (impl IntoSimpleExpr, Order)> + std::fmt::Debug + Clone,
        Page: Into<u64> + Copy,
        PerPage: Into<u64> + Copy,
    >(
        conn: &Connection,
        warehouse_id: uuid::Uuid,
        filter: Filter,
        sort: Sort,
        page: Page,
        per_page: PerPage,
    ) -> Result<Vec<(warehouse_product::Model, product::Model)>, DbErr> {
        #[cfg(feature = "cache")]
        cache_mget!(
            conn,
            format!(
                "warehouse_products:{warehouse_id}-{filter:?}-{sort:?}-{}/{}",
                page.into(),
                per_page.into()
            ),
            (warehouse_product::Model, product::Model)
        );

        let warehouse = Self::find_warehouse_by_id(conn, warehouse_id).await?;
        match warehouse {
            Some(warehouse) => {
                let mut query = warehouse
                    .find_related(WarehouseProduct)
                    .find_also_related(Product)
                    .filter(filter.clone());
                for (column, order) in sort.clone() {
                    query = query.order_by_with_nulls(column, order, sea_query::NullOrdering::Last);
                }
                let query = query.paginate(&conn.db_connection, per_page.into());

                let result = query.fetch_page(page.into()).await?;
                let result = result
                    .into_iter()
                    .filter_map(|(warehouse_product, product)| {
                        let product = product?;
                        Some((warehouse_product, product))
                    })
                    .collect::<Vec<_>>();

                #[cfg(feature = "cache")]
                cache_mset!(
                    conn,
                    format!(
                        "warehouse_products:{warehouse_id}-{filter:?}-{sort:?}-{}/{}",
                        page.into(),
                        per_page.into()
                    ),
                    result,
                    60 * 60 * 3,
                    |x: &(warehouse_product::Model, product::Model)| format!(
                        "warehouse_product:{}/{}",
                        x.0.warehouse_id, x.0.product_id
                    )
                );

                Ok(result)
            }
            // Maybe Add Error here instead
            None => Ok(Vec::new()),
        }
    }

    pub async fn count_warehouse_products_with_condition<Filter: sea_query::IntoCondition>(
        conn: &Connection,
        id: uuid::Uuid,
        filter: Filter,
    ) -> Result<u64, DbErr> {
        let warehouse = Self::find_warehouse_by_id(conn, id).await?;
        match warehouse {
            Some(warehouse) => Ok(warehouse
                .find_related(Product)
                .filter(filter)
                .count(&conn.db_connection)
                .await?),
            None => Ok(0),
        }
    }

    pub async fn find_warehouse_recipes_by_id<
        Filter: sea_query::IntoCondition + std::fmt::Debug + Clone,
        Sort: IntoIterator<Item = (impl IntoSimpleExpr, Order)> + std::fmt::Debug + Clone,
        Page: Into<u64> + Copy,
        PerPage: Into<u64> + Copy,
    >(
        conn: &Connection,
        warehouse_id: uuid::Uuid,
        filter: Filter,
        sort: Sort,
        page: Page,
        per_page: PerPage,
    ) -> Result<Vec<(warehouse_recipe::Model, recipe::Model)>, DbErr> {
        #[cfg(feature = "cache")]
        cache_mget!(
            conn,
            format!(
                "warehouse_recipes:{warehouse_id}-{filter:?}-{sort:?}-{}/{}",
                page.into(),
                per_page.into()
            ),
            (warehouse_recipe::Model, recipe::Model)
        );

        let warehouse = Self::find_warehouse_by_id(conn, warehouse_id).await?;
        match warehouse {
            Some(warehouse) => {
                let mut query = warehouse
                    .find_related(WarehouseRecipe)
                    .find_also_related(Recipe)
                    .filter(filter.clone());
                for (column, order) in sort.clone() {
                    query = query.order_by_with_nulls(column, order, sea_query::NullOrdering::Last);
                }
                let query = query.paginate(&conn.db_connection, per_page.into());

                let result = query.fetch_page(page.into()).await?;
                let result = result
                    .into_iter()
                    .filter_map(|(warehouse_recipe, recipe)| {
                        let recipe = recipe?;
                        Some((warehouse_recipe, recipe))
                    })
                    .collect::<Vec<_>>();

                #[cfg(feature = "cache")]
                cache_mset!(
                    conn,
                    format!(
                        "warehouse_recipes:{warehouse_id}-{filter:?}-{sort:?}-{}/{}",
                        page.into(),
                        per_page.into()
                    ),
                    result,
                    60 * 60 * 3,
                    |x: &(warehouse_recipe::Model, recipe::Model)| format!(
                        "warehouse_recipe:{}/{}",
                        x.0.warehouse_id, x.0.recipe_id
                    )
                );

                Ok(result)
            }
            // Maybe Add Error here instead
            None => Ok(Vec::new()),
        }
    }

    pub async fn count_warehouse_recipes_with_condition<Filter: sea_query::IntoCondition>(
        conn: &Connection,
        id: uuid::Uuid,
        filter: Filter,
    ) -> Result<u64, DbErr> {
        let warehouse = Self::find_warehouse_by_id(conn, id).await?;
        match warehouse {
            Some(warehouse) => Ok(warehouse
                .find_related(Recipe)
                .filter(filter)
                .count(&conn.db_connection)
                .await?),
            None => Ok(0),
        }
    }
}
