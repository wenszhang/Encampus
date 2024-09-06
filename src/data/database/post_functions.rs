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
    pub resolved: bool,
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
        "select title, postid as post_id, resolved from posts where posts.classid = $1 ORDER BY timestamp;",
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
pub async fn add_post(new_post_info: AddPostInfo, user_id: i32) -> Result<Post, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let post: Post = sqlx::query_as("INSERT INTO posts(timestamp, title, contents, authorid, anonymous, limitedvisibility, classid, resolved) VALUES(CURRENT_TIMESTAMP, $1, $2, $3, $4, $5, $6, false)
                        RETURNING                 
                        title, 
                        postid as post_id,
                        resolved;")
        .bind(new_post_info.title)
        .bind(new_post_info.contents)
        .bind(user_id)
        .bind(new_post_info.anonymous)
        .bind(new_post_info.limited_visibility)
        .bind(new_post_info.classid)
        .fetch_one(&pool)
        .await
        .expect("failed adding post");

    Ok(post)
}

#[server(ResolvePost)]
pub async fn resolve_post(post_id: i32, status: bool) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    sqlx::query("update posts set resolved = $1 where postid = $2")
        .bind(status)
        .bind(post_id)
        .execute(&pool)
        .await
        .expect("Cannot resolve post");

    Ok(())
}
