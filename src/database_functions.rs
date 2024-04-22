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
 * Struct to hold the post info
 */
#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow)]
pub struct Post(String);

/**
 * Struct to hold the class name
 */
#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow)]
pub struct ClassName(String);

/**
 * Struct to hold user
 */
#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow)]
pub struct User {
    pub name: String,
    pub id: i32,
}

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
 * Get all posts for a class given the class id
 */
#[server(GetPosts)]
pub async fn get_posts(class_id: i32) -> Result<Vec<String>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let rows: Vec<Post> = sqlx::query_as( "select title from posts join classes on posts.classid = classes.courseid where classes.courseid = $1")
        .bind(class_id)
        .fetch_all(&pool)
        .await
        .expect("select should work");

    let post_titles: Vec<String> = rows.into_iter().map(|row| row.0).collect();
    Ok(post_titles)
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

/**
 * Add a student to the system
 */
#[server(AddStudent)]
pub async fn add_student(name: String) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    sqlx::query("insert into users(name) values($1)")
        .bind(name.clone())
        .execute(&pool)
        .await
        .expect("msg");

    sqlx::query("insert into students(name) values($1)")
        .bind(name.clone())
        .execute(&pool)
        .await
        .expect("msg");

    Ok(())
}

/**
 * Check if a user exists
 */
#[server(CheckUser)]
pub async fn check_user(name: String) -> Result<(String, i32), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let user_result: User = sqlx::query_as("select * from users where name = $1")
        .bind(name)
        .fetch_one(&pool)
        .await
        .expect("No user found");

    // let user = User {
    //     name: user_result.0,
    //     id: user_result.1,
    // };

    Ok((user_result.name, user_result.id))
}
