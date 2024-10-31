use leptos::*;
use leptos::{server, ServerFnError};

use crate::data::database::class_functions::{check_user_is_instructor, ClassId};
use crate::data::database::user_functions::UserId;
use crate::pages::view_class_posts::focused_post::{AddReplyInfo, Reply};

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

#[server(ApproveReply)]
pub async fn approve_reply(reply_id: i32, user_id: i32, status: bool) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let ClassId(class_id) = sqlx::query_as(
        "select classid from posts where postid = (select postid from replies where replyid = $1)",
    )
    .bind(reply_id)
    .fetch_one(&pool)
    .await
    .expect("Cannot get class id");

    if check_user_is_instructor(user_id, class_id).await.unwrap() {
        sqlx::query("update replies set approved = $2 where replyid = $1")
            .bind(reply_id)
            .bind(status)
            .execute(&pool)
            .await
            .expect("Cannot approve reply");
    } else {
        return Err(ServerFnError::<NoCustomError>::ServerError(
            "User does not have access to approve".to_string(),
        ));
    }

    Ok(())
}

#[server(AddReply)]
pub async fn add_reply(reply_info: AddReplyInfo, user: String) -> Result<Reply, ServerFnError> {
    use crate::data::database::user_functions::UserId;
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to add Reply".to_string(),
    ))?;

    let user_id: UserId = sqlx::query_as("select id from users where username = $1")
        .bind(user)
        .fetch_one(&pool)
        .await
        .expect("select should work");

    let newreply: Reply = sqlx::query_as(
        "INSERT INTO replies (time, authorid, postid, anonymous, contents) 
                        VALUES (CURRENT_TIMESTAMP, $1, $2, $3, $4)
                RETURNING                 
                time, 
                contents,
                'You' as author_name, 
                authorid as author_id,
                anonymous,
                replyid as reply_id,
                removed,
                approved;",
    )
    .bind(user_id.0)
    .bind(reply_info.post_id)
    .bind(reply_info.anonymous)
    .bind(reply_info.contents)
    .fetch_one(&pool)
    .await
    .map_err(|db_error| {
        logging::error!(
            "\nAdd Reply Server Function Failed. Database returned error {:?}\n",
            db_error
        );
        ServerFnError::<NoCustomError>::ServerError("Unable to add Reply".to_string())
    })?;

    Ok(newreply)
}
