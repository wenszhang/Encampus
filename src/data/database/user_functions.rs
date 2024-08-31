use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};
/**
 * Struct to hold user
 */
#[derive(Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct User {
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub id: i32,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct UserId(pub i32);

// #[cfg(feature = "ssr")]
// #[derive(sqlx::FromRow)]
// pub struct FirstName(pub String);

// #[server(GetUserInfo)]
// pub async fn get_user_info(username: String) -> Result<String, ServerFnError> {
//     use leptos::{server_fn::error::NoCustomError, use_context};
//     use sqlx::postgres::PgPool;

//     let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
//         "Can't find user".to_string(),
//     ))?;

//     let FirstName(current_user) = sqlx::query_as("SELECT firstname FROM users where username = $1")
//         .bind(username)
//         .fetch_one(&pool)
//         .await
//         .expect("Can't find user");

//     Ok(current_user)
// }
