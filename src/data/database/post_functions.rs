#[allow(unused_imports)] // Suppress UserID - false compiler warning due to RowToStruct
use super::user_functions::UserId;
use crate::pages::view_class_posts::create_post::AddPostInfo;
use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

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
 * Get all posts for a class given the class id
 */
#[server(GetPosts)]
pub async fn get_posts(class_id: i32) -> Result<Vec<Post>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let rows: Vec<Post> = sqlx::query_as(
        "select title, postid as post_id from posts where posts.classid = $1 ORDER BY timestamp;",
    )
    .bind(class_id)
    .fetch_all(&pool)
    .await
    .expect("select should work");

    Ok(rows)
}

/**
 * Add a post to a class
 */
#[server(AddPost)]
pub async fn add_post(new_post_info: AddPostInfo, user: String) -> Result<Post, ServerFnError> {
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

    let post = sqlx::query_as("INSERT INTO posts(timestamp, title, contents, authorid, anonymous, limitedvisibility, classid) VALUES(CURRENT_TIMESTAMP, $1, $2, $3, $4, $5, $6)
                        RETURNING                 
                        title, 
                        postid as post_id;")
        .bind(new_post_info.title)
        .bind(new_post_info.contents)
        .bind(user_id.0)
        .bind(new_post_info.anonymous)
        .bind(new_post_info.limited_visibility)
        .bind(new_post_info.classid)
        .fetch_one(&pool)
        .await
        .expect("failed adding post");

    Ok(post)
}

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
