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
pub async fn update_user(user: User) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    sqlx::query(
        "update users set username = $1, firstname = $2, lastname = $3, role = $4 where id = $5",
    )
    .bind(user.username.clone())
    .bind(user.firstname.clone())
    .bind(user.lastname.clone())
    .bind(user.role.clone())
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
