use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

use crate::{data::global_state::User, pages::register_page::NewUser};
/**
 * Struct to hold user
 */
#[derive(Clone, Serialize, Deserialize, Default, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct DbUser {
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
pub struct UserPassword(String);

/**
 * Login a user or sign them up if they don't exist
 */
#[server(LoginSignUp)]
pub async fn login(username: String, password: String) -> Result<Option<DbUser>, ServerFnError> {
    use crate::AuthenticationSession;
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;
    let auth_session =
        use_context::<AuthenticationSession>().expect("Authentication Session should exist");

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let user_result: DbUser = sqlx::query_as(
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
        auth_session
            .login(User {
                id: user_result.id,
                first_name: user_result.firstname.clone(),
                last_name: user_result.lastname.clone(),
                user_name: user_result.username.clone(),
                role: user_result.role.clone(),
            })
            .await;

        Ok(Some(user_result))
    } else {
        Err(ServerFnError::<NoCustomError>::ServerError(
            "Incorrect Password".to_string(),
        ))
    }
}

#[server(Logout)]
pub async fn logout() -> Result<(), ServerFnError> {
    use leptos::use_context;
    let auth_session =
        use_context::<crate::AuthenticationSession>().expect("Authentication Session should exist");
    auth_session.logout().await;
    leptos_axum::redirect("/login");
    Ok(())
}

#[server(AddUser)]
pub async fn add_user(new_user: NewUser, login_new_user: bool) -> Result<DbUser, ServerFnError> {
    use crate::AuthenticationSession;
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    // let new_student = new_user.clone();

    if new_user.clone().user.role == "Student" {
        sqlx::query("insert into students(id, name) values($1, $2)")
            .bind(new_user.clone().user.id)
            .bind(new_user.clone().user.user_name)
            .execute(&pool)
            .await
            .expect("no users found");
    }

    let user: DbUser = sqlx::query_as(
        "insert into users(username, firstname, lastname, role, password) values($1, $2, $3, $4, $5) 
        returning username,
        firstname,
        lastname,   
        id,
        role",
    )
    .bind(new_user.clone().user.user_name)
    .bind(new_user.user.first_name)
    .bind(new_user.user.last_name)
    .bind(new_user.user.role)
    .bind(new_user.password)
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

    if login_new_user {
        let auth_session =
            use_context::<AuthenticationSession>().expect("Authentication Session should exist");
        auth_session
            .login(User {
                id: user.id,
                first_name: user.firstname.clone(),
                last_name: user.lastname.clone(),
                user_name: user.username.clone(),
                role: user.role.clone(),
            })
            .await;
    }
    Ok(user)
}

#[server(DeleteUser)]
pub async fn delete_user(user: User) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    if user.role == *"student" {
        sqlx::query("delete from students where id = $1")
            .bind(user.id)
            .execute(&pool)
            .await
            .map_err(|_| {
                ServerFnError::<NoCustomError>::ServerError("Unable to delete user".to_string())
            })?;
    } else if user.role == *"professor" {
        sqlx::query("delete from professors where id = $1")
            .bind(user.id)
            .execute(&pool)
            .await
            .map_err(|_| {
                ServerFnError::<NoCustomError>::ServerError("Unable to delete user".to_string())
            })?;
    }

    sqlx::query("delete from users where id = $1")
        .bind(user.id)
        .execute(&pool)
        .await
        .map_err(|_| {
            ServerFnError::<NoCustomError>::ServerError("Unable to delete user".to_string())
        })?;

    Ok(())
}

#[server(UpdateUser)]
pub async fn update_user(new_user: NewUser) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    sqlx::query(
        "update users set username = $1, firstname = $2, lastname = $3, role = $4, password = $5 where id = $6",
    )
    .bind(new_user.user.user_name.clone())
    .bind(new_user.user.first_name.clone())
    .bind(new_user.user.last_name.clone())
    .bind(new_user.user.role.clone())
    .bind(new_user.password)
    .bind(new_user.user.id)
    .execute(&pool)
    .await
    .map_err(|_| {
        ServerFnError::<NoCustomError>::ServerError("Unable to update user".to_string())
    })?;

    Ok(())
}

#[server(UpdateUserWithoutPassword)]
pub async fn update_user_without_password(user: User) -> Result<DbUser, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let user = sqlx::query_as(
        "update users set username = $1, firstname = $2, lastname = $3, role = $4 where id = $5 returning id, username, firstname, lastname, role",
    )
    .bind(user.user_name)
    .bind(user.first_name)
    .bind(user.last_name)
    .bind(user.role)
    .bind(user.id)
    .fetch_one(&pool)
    .await
    .map_err(|_| {
        ServerFnError::<NoCustomError>::ServerError("Unable to update user".to_string())
    })?;

    Ok(user)
}

#[server(GetUsers)]
pub async fn get_users() -> Result<Vec<DbUser>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let users: Vec<DbUser> =
        sqlx::query_as("select username, firstname, lastname, id, role from users order by role")
            .fetch_all(&pool)
            .await
            .expect("no users found");
    Ok(users)
}

#[server(GetUserById)]
pub async fn get_user_by_id(user_id: i32) -> Result<DbUser, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let user: DbUser =
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
pub async fn get_users_by_role(role: String) -> Result<Vec<DbUser>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let users: Vec<DbUser> =
        sqlx::query_as("select username, firstname, lastname, id, role from users where role = $1")
            .bind(role)
            .fetch_all(&pool)
            .await
            .expect("no users found");
    Ok(users)
}

#[server(GetUserPassword)]
pub async fn get_user_password(id: i32) -> Result<String, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let UserPassword(password) = sqlx::query_as("select password from users where id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .expect("No user found");

    Ok(password)
}

#[server(UpdatePassword)]
pub async fn update_user_password(user_id: i32, password: String) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    sqlx::query("UPDATE users SET password = $1 WHERE id = $2")
        .bind(password)
        .bind(user_id)
        .execute(&pool)
        .await
        .map_err(|_| {
            ServerFnError::<NoCustomError>::ServerError("Unable to update password".to_string())
        })?;

    Ok(())
}

// Unused
pub fn validate_password(password: &str) -> bool {
    let min_length = 8;
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special_char = password.chars().any(|c| !c.is_alphanumeric());

    password.len() >= min_length && has_uppercase && has_lowercase && has_digit && has_special_char
}
