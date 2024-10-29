use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

use crate::data::database::user_functions::UserId;

#[server(RemoveReply)]
pub async fn remove_reply(reply_id: i32, user_id: i32) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let UserId(instructor_id) = sqlx::query_as("select instructorid from classes where courseid = (select classid from posts where postid = (select postid from replies where replyid = $1))")
        .bind(reply_id)
        .fetch_one(&pool)
        .await
        .expect("Cannot get instructor id");

    let UserId(author_id) = sqlx::query_as("select authorid from replies where replyid = $1")
        .bind(reply_id)
        .fetch_one(&pool)
        .await
        .expect("Cannot get author id");

    if author_id == user_id || instructor_id == user_id {
        sqlx::query("update replies set removed = true where replyid = $1")
            .bind(reply_id)
            .execute(&pool)
            .await
            .expect("Cannot remove reply");
    }
    Ok(())
}
