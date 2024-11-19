use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Poll {
    pub id: i32,
    pub question: String,
    pub is_active: bool,
    pub course_id: i32,
}

/**
 * Struct to hold the poll info
 */
#[derive(Clone, Serialize, Deserialize, Default, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct PollInfo {
    pub poll_id: i32,
    pub poll_options: Vec<PollOption>,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct PollId(pub i32);

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct PollOption {
    pub id: i32,
    pub option_text: String,
}

#[server(CreatePoll)]
pub async fn create_poll(
    question: String,
    course_id: i32,
    options: Vec<String>,
) -> Result<PollInfo, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete request".to_string(),
    ))?;

    let PollId(poll_id) = sqlx::query_as(
        "insert into polls (question, is_active, course_id) 
         values ($1, $2, $3) 
         returning id",
    )
    .bind(question)
    .bind(true)
    .bind(course_id)
    .fetch_one(&pool)
    .await
    .map_err(|_| {
        ServerFnError::<NoCustomError>::ServerError("Unable to create poll".to_string())
    })?;

    // TODO: https://github.com/launchbadge/sqlx/blob/main/FAQ.md#how-can-i-bind-an-array-to-a-values-clause-how-can-i-do-bulk-inserts USE THIS TO ADD MULTIPLE
    let poll_options: Vec<PollOption> = sqlx::query_as(
        "insert into poll_options (poll_id, option_text) 
         select $1, * from unnest($2::text[]) 
         returning id, option_text",
    )
    .bind(poll_id)
    .bind(options)
    .fetch_all(&pool)
    .await
    .map_err(|_| {
        ServerFnError::<NoCustomError>::ServerError("Unable to create poll".to_string())
    })?;

    Ok(PollInfo {
        poll_id,
        poll_options,
    })
}

// For student to get the options of the poll. ALso need them to get the question of the poll. get question of the poll and options of the poll in one fucntion.
#[server(GetPollOptions)]
pub async fn get_poll_options(poll_id: i32) -> Result<Vec<PollOption>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete request".to_string(),
    ))?;

    let options: Vec<PollOption> = sqlx::query_as(
        "select id, poll_id, option_text, vote_count from poll_options where poll_id = $1",
    )
    .bind(poll_id)
    .fetch_all(&pool)
    .await
    .expect("No poll options found");

    Ok(options)
}

#[server(VoteOnPollOption)]
pub async fn vote_on_poll_option(poll_option_id: i32) -> Result<PollOption, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete request".to_string(),
    ))?;

    let option: PollOption = sqlx::query_as(
        "update poll_options set vote_count = vote_count + 1 WHERE id = $1 returning *",
    )
    .bind(poll_option_id)
    .fetch_one(&pool)
    .await
    .map_err(|_| {
        ServerFnError::<NoCustomError>::ServerError("Unable to vote on poll option".to_string())
    })?;

    Ok(option)
}

#[server(GetPollResults)]
pub async fn get_poll_results(poll_id: i32) -> Result<Vec<PollOption>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete request".to_string(),
    ))?;

    let options: Vec<PollOption> = sqlx::query_as(
        "select id, poll_id, option_text, vote_count from poll_options where poll_id = $1 order by vote_count desc",
    )
    .bind(poll_id)
    .fetch_all(&pool)
    .await
    .expect("No poll options found");

    Ok(options)
}

#[server(UpdatePoll)]
pub async fn update_poll(poll_id: i32, question: String) -> Result<Poll, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete request".to_string(),
    ))?;

    let poll: Poll = sqlx::query_as("update polls set question = $1 where id = $2 returning *")
        .bind(question)
        .bind(poll_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| {
            ServerFnError::<NoCustomError>::ServerError("Unable to update poll".to_string())
        })?;

    Ok(poll)
}

#[server(DeletePoll)]
pub async fn delete_poll(poll_id: i32) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete request".to_string(),
    ))?;

    sqlx::query("delete from polls where id = $1")
        .bind(poll_id)
        .execute(&pool)
        .await
        .map_err(|_| {
            ServerFnError::<NoCustomError>::ServerError("Unable to delete poll".to_string())
        })?;

    Ok(())
}

// Maybe make an get all polls?? May need other server functions

#[server(GetAllPolls)]
pub async fn get_all_polls() -> Result<Vec<Poll>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete request".to_string(),
    ))?;

    let polls: Vec<Poll> = sqlx::query_as("select * from polls order by created_at desc")
        .fetch_all(&pool)
        .await
        .map_err(|_| {
            ServerFnError::<NoCustomError>::ServerError("Unable to fetch polls".to_string())
        })?;

    Ok(polls)
}
