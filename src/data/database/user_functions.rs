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
    pub role: String,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct UserId(pub i32);

/**
 * Login a user or sign them up if they don't exist
 */
#[server(LoginSignUp)]
pub async fn login_signup(username: String) -> Result<User, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let mut user_result: Option<User> = sqlx::query_as(
        "select username, firstname, lastname, id, role from users where username = $1",
    )
    .bind(username.clone())
    .fetch_optional(&pool)
    .await?;

    if user_result.is_none() {
        sqlx::query(
            "insert into users(username, firstname, lastname, role) values($1, $1, $1, 'student')",
        )
        .bind(username.clone())
        .execute(&pool)
        .await
        .expect("Failed adding user");

        sqlx::query("insert into students(name) values($1)")
            .bind(username.clone())
            .execute(&pool)
            .await
            .expect("Failed adding student");

        user_result = sqlx::query_as(
            "select username, firstname, lastname, id, role from users where username = $1",
        )
        .bind(username.clone())
        .fetch_optional(&pool)
        .await?;
    }

    Ok(user_result.unwrap())
}
