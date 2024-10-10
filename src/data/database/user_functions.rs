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

/**
 * Login a user or sign them up if they don't exist
 */
#[server(LoginSignUp)]
pub async fn login(username: String) -> Result<User, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let user_result: Option<User> = sqlx::query_as(
        "select username, firstname, lastname, id, role from users where username = $1",
    )
    .bind(username.clone())
    .fetch_optional(&pool)
    .await?;

    // if user_result.is_none() {
    //     sqlx::query(
    //         "insert into users(username, firstname, lastname, role) values($1, $1, $1, 'student')",
    //     )
    //     .bind(username.clone())
    //     .execute(&pool)
    //     .await
    //     .expect("Failed adding user");

    //     sqlx::query("insert into students(name) values($1)")
    //         .bind(username.clone())
    //         .execute(&pool)
    //         .await
    //         .expect("Failed adding student");

    //     user_result = sqlx::query_as(
    //         "select username, firstname, lastname, id, role from users where username = $1",
    //     )
    //     .bind(username.clone())
    //     .fetch_optional(&pool)
    //     .await?;
    // }

    Ok(user_result.unwrap_or_else(|| User {
        username: "".to_string(),
        firstname: "".to_string(),
        lastname: "".to_string(),
        id: 0,
        role: "".to_string(),
    }))
}

#[server(AddUser)]
pub async fn add_user(user: User) -> Result<User, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let user: User = sqlx::query_as(
        "insert into users(username, firstname, lastname, role) values($1, $2, $3, 'student') 
        returning username,
        firstname,
        lastname,   
        id,
        role",
    )
    .bind(user.username.clone())
    .bind(user.firstname.clone())
    .bind(user.lastname.clone())
    .fetch_one(&pool)
    .await
    .map_err(|_| {
        ServerFnError::<NoCustomError>::ServerError("Unable to create user, username already exists".to_string())
    })?;

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
