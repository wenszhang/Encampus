use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

/**
 * Struct to hold user
 */
#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct User {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub id: i32,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct UserId(pub i32);

#[server(GetUserInfo)]
pub async fn get_user_info(username: String) -> Result<User, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let current_user: User =
        sqlx::query_as("select username, firstname, lastname, id from users where username = $1")
            .bind(username)
            .fetch_one(&pool)
            .await
            .expect("failed getting user");

    Ok(current_user)
}
