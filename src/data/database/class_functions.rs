/**
 * This file contains all the database functions that are used in the server
 */
use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

/**
 * Struct to hold the class info
 */
#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ClassInfo {
    pub id: i32,
    pub name: String,
}

/**
 * Struct to hold the class name
 */
#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow)]
pub struct ClassName(String);

#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow)]
pub struct UserName(String);

/**
 * Get all class names from the database
 * Will eventually have a user added and so query will be modified to get only the classes the user is registered to
 */
#[server(GetClassList)]
pub async fn get_class_list() -> Result<Vec<ClassInfo>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let classes: Vec<ClassInfo> =
        sqlx::query_as("SELECT courseid as id, coursename as name from classes")
            .fetch_all(&pool)
            .await
            .expect("select should work");

    Ok(classes)
}

/**
 * Get the class name given the class id
 */
#[server(GetClassName)]
pub async fn get_class_name(class_id: i32) -> Result<String, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let ClassName(name) = sqlx::query_as("select coursename from classes where courseid = $1")
        .bind(class_id)
        .fetch_one(&pool)
        .await
        .expect("select should work");
    Ok(name)
}

#[server(GetInstructor)]
pub async fn get_instructor(post_id: i32) -> Result<String, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let UserName (name)= sqlx::query_as(
        "select username from instructing join users on professorid = id where courseid = (select classid from posts where postid = $1);",
    )
    .bind(post_id)
    .fetch_one(&pool)
    .await
    .expect("select should work");
    Ok(name)
}
