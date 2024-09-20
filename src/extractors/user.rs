use crate::{error::AppError, models::oidc_user::OidcUser, models::user::User};
use axum::extract::{FromRef, FromRequestParts};
use service::Connection;

/// Extracts an `OidcUser` from the request parts.
///
/// This implementation enables the extraction of the `OidcUser` struct from incoming
/// HTTP request parts using Axum's `FromRequestParts` trait. The user information is
/// retrieved from the OpenID Connect (OIDC) claims, and the required fields (ID, username,
/// name, and email) are extracted from the OIDC token claims.
#[axum::async_trait]
impl<S> FromRequestParts<S> for User
where
    Connection: axum::extract::FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let conn = Connection::from_ref(state);
        let oidc_user = OidcUser::from_request_parts(parts, state).await?;

        let user = service::Query::find_user_by_id(&conn, oidc_user.id)
            .await?
            .ok_or(AppError::Unknow)?; // This sould never happen

        Ok(user.into())
    }
}
