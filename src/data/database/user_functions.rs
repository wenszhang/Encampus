use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};
/**
 * Struct to hold user
 */
#[derive(Clone, Serialize, Deserialize, Default, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct User {
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub id: i32,
    pub role: String,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct UserId(pub i32);

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct UserPassword(pub String);

/**
 * Login a user or sign them up if they don't exist
 */
#[server(LoginSignUp)]
pub async fn login(username: String, password: String) -> Result<User, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let user_result: User = sqlx::query_as(
        "select username, firstname, lastname, id, role from users where username = $1",
    )
    .bind(username.clone())
    .fetch_one(&pool)
    .await
    .expect("No user found");

    let UserPassword(user_password) =
        sqlx::query_as("select password from users where username = $1")
            .bind(username.clone())
            .fetch_one(&pool)
            .await
            .expect("No user found");

    if user_password == password {
        Ok(user_result)
    } else {
        Err(ServerFnError::<NoCustomError>::ServerError(
            "Incorrect Password".to_string(),
        ))
    }
}

#[server(AddUser)]
pub async fn add_user(user: User) -> Result<i32, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let UserId(user_id) = sqlx::query_as(
        "insert into users(username, firstname, lastname, role) values($1, $2, $3, 'student') returning id",
    )
    .bind(user.username.clone())
    .bind(user.firstname.clone())
    .bind(user.lastname.clone())
    .fetch_one(&pool)
    .await
    .map_err(|_| {
        ServerFnError::<NoCustomError>::ServerError("Unable to create user, username already exists".to_string())
    })?;

    Ok(user_id)
}
