//! Location query services for the `scrounch_backend` application.
//!
//! This module defines services and functions related to querying location data in the application.
//! It encapsulates the logic for retrieving location information from the database based on various
//! criteria such as location ID, name, or category. These services provide a layer of abstraction
//! over database interactions, allowing for efficient and consistent data retrieval related to
//! locations.

use crate::{
    query::Query,
    r#macro::{cache_get, cache_mget, cache_mset, cache_set},
    Connection,
};
use ::entity::models::{location, location::Entity as Location};
use sea_orm::*;

impl Query {
    pub async fn find_location_by_id(
        conn: &Connection,
        id: uuid::Uuid,
    ) -> Result<Option<location::Model>, DbErr> {
        #[cfg(feature = "cache")]
        cache_get!(conn, format!("location:{id}"), location::Model);

        let result = Location::find_by_id(id).one(&conn.db_connection).await?;

        #[cfg(feature = "cache")]
        if let Some(model) = &result {
            cache_set!(conn, format!("location:{id}"), model, 60 * 15);
        }

        Ok(result)
    }

    pub async fn list_locations_with_condition<
        F: sea_query::IntoCondition + std::fmt::Debug + Clone,
        A: Into<u64> + Copy,
        P: Into<u64> + Copy,
    >(
        conn: &Connection,
        filter: F,
        page: A,
        per_page: P,
    ) -> Result<Vec<location::Model>, DbErr> {
        #[cfg(feature = "cache")]
        cache_mget!(
            conn,
            format!("locations:{filter:?}-{}/{}", page.into(), per_page.into()),
            location::Model
        );

        let result = Location::find()
            .filter(filter.clone())
            .paginate(&conn.db_connection, per_page.into())
            .fetch_page(page.into())
            .await?;

        #[cfg(feature = "cache")]
        cache_mset!(
            conn,
            format!("locations:{filter:?}-{}/{}", page.into(), per_page.into()),
            result,
            60 * 15,
            "location:"
        );

        Ok(result)
    }

    pub async fn count_locations_with_condition<F: sea_query::IntoCondition>(
        conn: &Connection,
        filter: F,
    ) -> Result<u64, DbErr> {
        Location::find()
            .filter(filter)
            .count(&conn.db_connection)
            .await
    }
}
