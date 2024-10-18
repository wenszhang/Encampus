use leptos::server_fn::error::NoCustomError;
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
pub async fn add_user(new_user: User, password: String) -> Result<User, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let user: User = sqlx::query_as(
        "insert into users(username, firstname, lastname, role, password) values($1, $2, $3, $4, $5) 
        returning username,
        firstname,
        lastname,   
        id,
        role",
    )
    .bind(new_user.username.clone())
    .bind(new_user.firstname.clone())
    .bind(new_user.lastname.clone())
    .bind(new_user.role.clone())
    .bind(password)
    .fetch_one(&pool)
    .await
    .map_err(|_| {
        ServerFnError::<NoCustomError>::ServerError(
            "Unable to create user, username already exists".to_string(),
        )
    })?;

    if user.role == *"student" {
        let user = user.clone();
        sqlx::query("insert into students(id, name) values($1, $2)")
            .bind(user.id)
            .bind(user.username)
            .execute(&pool)
            .await
            .expect("no users found");
    } else if user.role == *"professor" {
        let user = user.clone();
        sqlx::query("insert into professors(id, name) values($1, $2)")
            .bind(user.id)
            .bind(user.username)
            .execute(&pool)
            .await
            .expect("no users found");
    }
    Ok(user)
}

#[server(UpdateUser)]
pub async fn update_user(user: User) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    sqlx::query("update users set username = $1, firstname = $2, lastname = $3 where id = $4")
        .bind(user.username.clone())
        .bind(user.firstname.clone())
        .bind(user.lastname.clone())
        .bind(user.id)
        .execute(&pool)
        .await
        .map_err(|_| {
            ServerFnError::<NoCustomError>::ServerError("Unable to update user".to_string())
        })?;

    Ok(())
}

#[server(GetUsers)]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let users: Vec<User> =
        sqlx::query_as("select username, firstname, lastname, id, role from users order by role")
            .fetch_all(&pool)
            .await
            .expect("no users found");
    Ok(users)
}

#[server(GetUserById)]
pub async fn get_user_by_id(user_id: i32) -> Result<User, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let user: User =
        sqlx::query_as("select username, firstname, lastname, id, role from users where id = $1")
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .map_err(|_| {
                ServerFnError::<NoCustomError>::ServerError("User not found".to_string())
            })?;

    Ok(user)
}

#[server(GetUsersByRole)]
pub async fn get_users_by_role(role: String) -> Result<Vec<User>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let users: Vec<User> =
        sqlx::query_as("select username, firstname, lastname, id, role from users where role = $1")
            .bind(role)
            .fetch_all(&pool)
            .await
            .expect("no users found");
    Ok(users)
}

#[server(UpdateUserCredentials)]
pub async fn update_user_credentials(
    user_id: i32,
    username: String,
    password: String,
) -> Result<(), ServerFnError<NoCustomError>> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    sqlx::query("UPDATE users SET username = $1, password = $2 WHERE id = $3")
        .bind(username)
        .bind(password)
        .bind(user_id)
        .execute(&pool)
        .await
        .map_err(|_| {
            ServerFnError::<NoCustomError>::ServerError(
                "Unable to update user credentials".to_string(),
            )
        })?;

    Ok(())
}

pub fn validate_password(password: &str) -> bool {
    let min_length = 8;
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    let has_special_char = password.chars().any(|c| !c.is_alphanumeric());

    password.len() >= min_length && has_uppercase && has_lowercase && has_digit && has_special_char
}
