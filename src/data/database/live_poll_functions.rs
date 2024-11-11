use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Poll {
    pub id: i32,
    pub question: String,
    pub created_at: chrono::NaiveDateTime,
    pub is_active: bool,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct PollOption {
    pub id: i32,
    pub poll_id: i32,
    pub option_text: String,
    pub vote_count: i32,
}

#[server(CreatePoll)]
pub async fn create_poll(question: String) -> Result<Poll, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let poll: Poll = sqlx::query_as(
        "INSERT INTO polls (question, created_at, is_active) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(question)
    .bind(chrono::Utc::now().naive_utc())
    .bind(true)
    .fetch_one(&pool)
    .await
    .map_err(|_| {
        ServerFnError::<NoCustomError>::ServerError("Unable to create poll".to_string())
    })?;

    Ok(poll)
}

#[server(GetPollOptions)]
pub async fn get_poll_options(poll_id: i32) -> Result<Vec<PollOption>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let options: Vec<PollOption> = sqlx::query_as(
        "SELECT id, poll_id, option_text, vote_count FROM poll_options WHERE poll_id = $1",
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
        "Unable to complete Request".to_string(),
    ))?;

    let option: PollOption = sqlx::query_as(
        "UPDATE poll_options SET vote_count = vote_count + 1 WHERE id = $1 RETURNING *",
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
        "Unable to complete Request".to_string(),
    ))?;

    let options: Vec<PollOption> = sqlx::query_as(
        "SELECT id, poll_id, option_text, vote_count FROM poll_options WHERE poll_id = $1 ORDER BY vote_count DESC",
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
        "Unable to complete Request".to_string(),
    ))?;

    let poll: Poll = sqlx::query_as("UPDATE polls SET question = $1 WHERE id = $2 RETURNING *")
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
        "Unable to complete Request".to_string(),
    ))?;

    sqlx::query("DELETE FROM polls WHERE id = $1")
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
        "Unable to complete Request".to_string(),
    ))?;

    let polls: Vec<Poll> = sqlx::query_as("SELECT * FROM polls ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
        .map_err(|_| {
            ServerFnError::<NoCustomError>::ServerError("Unable to fetch polls".to_string())
        })?;

    Ok(polls)
}
