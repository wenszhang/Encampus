use crate::data::database::post_functions::Post;
use leptos::{server, ServerFnError};

#[server(FilterPosts)]
pub async fn filter_posts(
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
