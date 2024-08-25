use chrono::DateTime;
/**
 * This file contains all the database functions that are used in the server
 */
use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

/**
 * Struct to hold the class info
 */
#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct AnnouncementInfo {
    pub announcement_id: i32,
    pub time: chrono::NaiveDateTime,
    pub title: String,
    pub contents: String,
    pub class_id: i32,
    pub author_id: i32,
}

/**
 * Get all announcements from the database
 *
 */
#[server(GetAnnouncementsList)]
pub async fn get_announcement_list(class_id: i32) -> Result<Vec<AnnouncementInfo>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let announcements: Vec<AnnouncementInfo> =
        sqlx::query_as("SELECT announcementid as annoucement_id, time, title, contents, classid as class_id, authorid as author_id from announcements where classid = $1")
            .bind(class_id)
            .fetch_all(&pool)
            .await
            .expect("select should work");

    Ok(announcements)
}
