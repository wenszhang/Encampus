#[allow(unused_imports)] // Suppress UserID - false compiler warning due to RowToStruct
use super::user_functions::UserId;
use crate::pages::view_class_posts::create_post::AddPostInfo;
use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

/**
 * Struct to hold information for displaying a post in a list
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Post {
    pub title: String,
    pub post_id: i32,
    pub resolved: bool,
    pub private: bool,
    pub author_id: i32,
}

#[derive(PartialEq, Clone, Copy)]
pub struct PostFetcher {
    pub class_id: i32,
    pub user_id: i32,
}

/**
 * Get all posts for a class given the class id
 */
#[server(GetPosts)]
pub async fn get_posts(class_id: i32, user_id: i32) -> Result<Vec<Post>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let rows: Vec<Post> = sqlx::query_as(
        "select title, postid as post_id, resolved, private, authorid as author_id from posts where removed = false and ((posts.classid = $1 and private = false) or (posts.classid = $1 and authorid = $2 and private = true) or (classid = $1 and (select instructorid from classes where courseid = $1) = $2)) ORDER BY timestamp desc;",
    )
    .bind(class_id)
    .bind(user_id)
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

    let post: Post = sqlx::query_as("INSERT INTO posts(timestamp, title, contents, authorid, anonymous, limitedvisibility, classid, resolved, private) VALUES(CURRENT_TIMESTAMP, $1, $2, $3, $4, $5, $6, false, $7)
                        RETURNING                
                        title, 
                        postid as post_id,
                        resolved,
                        authorid as author_id,
                        private;")
        .bind(new_post_info.title)
        .bind(new_post_info.contents)
        .bind(user_id)
        .bind(new_post_info.anonymous)
        .bind(new_post_info.limited_visibility)
        .bind(new_post_info.classid)
        .bind(new_post_info.private)
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

#[server(RemovePost)]
pub async fn remove_post(post_id: i32, user_id: i32) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let UserId(instructor_id) = sqlx::query_as("select instructorid from classes where courseid = (select classid from posts where postid = $1)")
        .bind(post_id)
        .fetch_one(&pool)
        .await
        .expect("Cannot get instructor id");

    let UserId(author_id) = sqlx::query_as("select authorid from posts where postid = $1")
        .bind(post_id)
        .fetch_one(&pool)
        .await
        .expect("Cannot get author id");

    if author_id == user_id || instructor_id == user_id {
        sqlx::query("update posts set removed = true where postid = $1")
            .bind(post_id)
            .execute(&pool)
            .await
            .expect("Cannot remove post");
    }
    Ok(())
}

#[server(FilterPosts)]
pub async fn get_search_posts(
    class_id: i32,
    user_id: i32,
    filter_keyword: String,
) -> Result<Vec<Post>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let posts: Vec<Post> = sqlx::query_as("select title, postid as post_id, resolved, private, authorid as author_id from posts where to_tsvector(title || ' ' || contents) @@ to_tsquery($3) and classid = $1 and removed = false and ((posts.classid = $1 and private = false) or (posts.classid = $1 and authorid = $2 and private = true) or (classid = $1 and (select instructorid from classes where courseid = $1) = $2)) ORDER BY timestamp desc")
        .bind(class_id)
        .bind(user_id)
        .bind(filter_keyword)
        .fetch_all(&pool)
        .await
        .expect("No posts found");

    Ok(posts)
}
