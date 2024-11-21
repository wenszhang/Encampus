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

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Answer {
    pub pollid: i32,
    pub answer: String,
    pub voted_count: i32,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct StudentAnswer {
    pub user_id: i32,
    pub pollid: i32,
    pub answer: String,
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
    answers: Vec<String>,
) -> Result<Poll, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete request".to_string(),
    ))?;

    // Insert poll
    let poll_id = sqlx::query_scalar::<_, i32>(
        "INSERT INTO polls (question, is_active, course_id)
         VALUES ($1, $2, $3)
         RETURNING id",
    )
    .bind(&question)
    .bind(true)
    .bind(course_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;

    // Insert answers
    for answer_text in answers {
        sqlx::query(
            "INSERT INTO answers (pollid, answer, voted_count)
             VALUES ($1, $2, $3)",
        )
        .bind(poll_id)
        .bind(&answer_text)
        .bind(0)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;
    }

    // Fetch the created poll
    let poll: Poll = sqlx::query_as("SELECT * FROM polls WHERE id = $1")
        .bind(poll_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;

    Ok(poll)
}

#[server(GetPollAnswers)]
pub async fn get_poll_answers(poll_id: i32) -> Result<Vec<Answer>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete request".to_string(),
    ))?;

    let answers: Vec<Answer> = sqlx::query_as::<_, Answer>(
        "SELECT pollid, answer, voted_count FROM answers WHERE pollid = $1",
    )
    .bind(poll_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;

    Ok(answers)
}

#[server(VoteOnPollAnswer)]
pub async fn vote_on_poll_answer(
    user_id: i32,
    poll_id: i32,
    new_answer: String,
    old_answer: Option<String>,
) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::{postgres::PgPool, Executor}; // Ensure Executor is imported

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;

    // Borrow old_answer to avoid moving it
    if let Some(ref old_answer_value) = old_answer {
        sqlx::query(
            "UPDATE answers SET voted_count = voted_count - 1
             WHERE pollid = $1 AND answer = $2",
        )
        .bind(poll_id)
        .bind(old_answer_value)
        .execute(&mut *tx)
        .await
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;
    }

    // Now old_answer can still be used here
    if old_answer.is_some() {
        sqlx::query(
            "UPDATE student_answers SET answer = $1
             WHERE user_id = $2 AND pollid = $3",
        )
        .bind(&new_answer)
        .bind(user_id)
        .bind(poll_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;
    } else {
        sqlx::query(
            "INSERT INTO student_answers (user_id, pollid, answer)
             VALUES ($1, $2, $3)",
        )
        .bind(user_id)
        .bind(poll_id)
        .bind(&new_answer)
        .execute(&mut *tx)
        .await
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;
    }

    tx.commit()
        .await
        .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;

    Ok(())
}

#[server(GetStudentAnswer)]
pub async fn get_student_answer(
    user_id: i32,
    poll_id: i32,
) -> Result<Option<String>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete request".to_string(),
    ))?;

    let result: Option<(String,)> =
        sqlx::query_as("SELECT answer FROM student_answers WHERE user_id = $1 AND pollid = $2")
            .bind(user_id)
            .bind(poll_id)
            .fetch_optional(&pool)
            .await
            .map_err(|e| ServerFnError::<NoCustomError>::ServerError(e.to_string()))?;

    Ok(result.map(|(answer,)| answer))
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
#[server(GetPollById)]
pub async fn get_poll_by_id(poll_id: i32) -> Result<Poll, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete request".to_string(),
    ))?;
    let poll = sqlx::query_as::<_, Poll>("SELECT * FROM polls WHERE id = $1")
        .bind(poll_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| {
            ServerFnError::<NoCustomError>::ServerError("Unable to update poll".to_string())
        })?;
    Ok(poll)
}
//This is only called at the end. Do not call it for anyone but professor, but call it for everyone when exiting the eventLoop
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
pub async fn update_poll(
    poll_id: i32,
    new_answer: String,
    old_answer: Option<String>,
) -> Result<Poll, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;
    //Check if it's a first time, or update for a poll answer

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete request".to_string(),
    ))?;
    //DB is still called answer, but param is new_answer for brevity sake
    let poll: Poll = sqlx::query_as("update polls set question = $1 where id = $2 returning *")
        .bind(new_answer)
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
pub async fn get_all_polls(course_id: i32) -> Result<Vec<Poll>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::{postgres::PgPool, Row};

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete request".to_string(),
    ))?;

    let polls: Vec<Poll> = sqlx::query_as("SELECT * FROM polls WHERE course_id = $1")
        .bind(course_id)
        .fetch_all(&pool)
        .await
        .map_err(|_| {
            ServerFnError::<NoCustomError>::ServerError("Unable to delete poll".to_string())
        })?;

    Ok(polls)
}
