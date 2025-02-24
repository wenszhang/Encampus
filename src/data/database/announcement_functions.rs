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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddAnnouncementInfo {
    pub title: String,
    pub contents: String,
    pub class_id: i32,
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
        sqlx::query_as("SELECT announcementid as announcement_id, time, title, contents, classid as class_id, authorid as author_id from announcements where classid = $1")
            .bind(class_id)
            .fetch_all(&pool)
            .await
            .expect("select should work");

    Ok(announcements)
}

#[server(PostAnnouncement)]
pub async fn post_announcement(
    new_announcement_info: AddAnnouncementInfo,
    user_id: i32,
) -> Result<AnnouncementInfo, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let announcement: AnnouncementInfo = sqlx::query_as(
        "INSERT INTO announcements (classid, authorid, title, contents, time)
         VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
            RETURNING
            announcementid as announcement_id,
            time,
            title,
            contents,
            classid as class_id,
            authorid as author_id",
    )
    .bind(new_announcement_info.class_id)
    .bind(user_id)
    .bind(new_announcement_info.title)
    .bind(new_announcement_info.contents)
    .fetch_one(&pool)
    .await
    .expect("failed adding announcement");

    Ok(announcement)
}

#[server(GetAnnouncementByID)]
pub async fn get_announcement_by_id(
    announcement_id: i32,
) -> Result<AnnouncementInfo, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    // Access the database connection pool
    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    // Fetch the specific announcement by announcement_id
    let announcement: AnnouncementInfo =
        sqlx::query_as("SELECT announcementid as announcement_id, time, title, contents, classid as class_id, authorid as author_id
                        FROM announcements
                        WHERE announcementid = $1")
            .bind(announcement_id)
            .fetch_one(&pool)
            .await
            .expect("select should work");

    Ok(announcement)
}

#[server(DeleteAnnouncement)]
pub async fn delete_announcement(announcement_id: i32) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    // Access the database connection pool
    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    // Delete the announcement with the given ID
    sqlx::query("DELETE FROM announcements WHERE announcementid = $1")
        .bind(announcement_id)
        .execute(&pool)
        .await
        .map_err(|_| {
            ServerFnError::<NoCustomError>::ServerError("Failed to delete announcement".to_string())
        })?;

    Ok(())
}
