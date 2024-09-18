use crate::data::database::post_functions::Post;
use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

#[server(FilterPosts)]
pub async fn filter_posts(
    class_id: i32,
    filter_keyword: String,
) -> Result<Vec<Post>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let posts: Vec<Post> = sqlx::query_as("select title, postid as post_id, resolved, private, authorid as author_id from posts where to_tsvector(title || ' ' || contents) @@ to_tsquery($1) and classid = $2")
        .bind(filter_keyword)
        .bind(class_id)
        .fetch_all(&pool)
        .await
        .expect("No posts found");

    Ok(posts)
}
