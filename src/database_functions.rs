use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

use crate::components::create_post::AddPostInfo;

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
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Post {
    pub title: String,
    pub post_id: i32,
}

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
pub async fn get_posts(class_id: i32) -> Result<Vec<Post>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let rows: Vec<Post> =
        sqlx::query_as("select title, postid as post_id from posts where posts.classid = $1")
            .bind(class_id)
            .fetch_all(&pool)
            .await
            .expect("select should work");

    Ok(rows)
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
 * Login a user or sign them up if they don't exist
 */
#[server(LoginSignUp)]
pub async fn login_signup(name: String) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let user_result: Option<User> = sqlx::query_as("select name, id from users where name = $1")
        .bind(name.clone())
        .fetch_optional(&pool)
        .await?;

    if user_result.is_none() {
        sqlx::query("insert into users(name) values($1)")
            .bind(name.clone())
            .execute(&pool)
            .await
            .expect("Failed adding user");

        sqlx::query("insert into students(name) values($1)")
            .bind(name.clone())
            .execute(&pool)
            .await
            .expect("Failed adding student");
    }

    Ok(())
}

/**
 * Add a post to a class
 */
#[server(AddPost)]
pub async fn add_post(new_post_info: AddPostInfo, user: String) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let user_id: UserId = sqlx::query_as("select id from users where name = $1")
        .bind(user)
        .fetch_one(&pool)
        .await
        .expect("select should work");

    sqlx::query("INSERT INTO posts(timestamp, title, contents, authorid, anonymous, limitedvisibility, classid) VALUES(CURRENT_TIMESTAMP, $1, $2, $3, $4, $5, $6);")
        .bind(new_post_info.title)
        .bind(new_post_info.contents)
        .bind(user_id.0)
        .bind(new_post_info.anonymous)
        .bind(new_post_info.limited_visibility)
        .bind(new_post_info.classid)
        .execute(&pool)
        .await
        .expect("failed adding post");

    Ok(())
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct UserId(pub i32);

#[server(GetAuthorIDFromName)]
pub async fn get_author_id_from_name(name: String) -> Result<i32, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let UserId(user) = sqlx::query_as("select id from users where name = $1")
        .bind(name)
        .fetch_one(&pool)
        .await
        .expect("select should work");

    Ok(user)
}
