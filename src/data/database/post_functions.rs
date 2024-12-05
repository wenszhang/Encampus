#[allow(unused_imports)] // Suppress UserID - false compiler warning due to RowToStruct
use super::user_functions::UserId;
use crate::pages::view_class_posts::create_post::AddPostInfo;
use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::logging::error;
        use leptos::{server_fn::error::NoCustomError, use_context};
        use sqlx::postgres::PgPool;
        use crate::data::database::class_functions::get_class_description;
        use crate::pages::view_class_posts::focused_post::AddReplyInfo;
        use crate::data::database::reply_functions::add_reply;
        use crate::data::database::ai_functions::get_gemini_response;
    }
}

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
    pub endorsed: bool,
    // pub pinned: bool,
    pub last_bumped: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
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
    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let rows: Vec<Post> = sqlx::query_as(
        "select title, postid as post_id, resolved, private, authorid as author_id, endorsed, last_bumped, timestamp, created_at
        from posts where removed = false
        and ((posts.classid = $1 and private = false)
            or (posts.classid = $1 and authorid = $2 and private = true)
            or (classid = $1 and (select instructorid from classes where courseid = $1) = $2))
        ORDER BY last_bumped desc, timestamp desc;",
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
    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let post: Post = sqlx::query_as("INSERT INTO posts(timestamp, title, contents, authorid, anonymous, limitedvisibility, classid, resolved, private) VALUES(CURRENT_TIMESTAMP, $1, $2, $3, $4, $5, $6, false, $7)
                        RETURNING                
                        title, 
                        postid as post_id,
                        resolved,
                        authorid as author_id,
                        private,
                        endorsed,
                        last_bumped,
                        created_at;")
        .bind(new_post_info.clone().title)
        .bind(new_post_info.clone().contents)
        .bind(user_id)
        .bind(new_post_info.clone().anonymous)
        .bind(new_post_info.clone().limited_visibility)
        .bind(new_post_info.clone().classid)
        .bind(new_post_info.clone().private)
        .fetch_one(&pool)
        .await
        .expect("failed adding post");

    if new_post_info.ai_response {
        let class_description = get_class_description(new_post_info.classid).await?;

        let ai_input = format!(
            "{}\n{}\n\n{}\n{}\n{}",
            "A question was asked in a class. Here is the description of the class:",
            class_description,
            "Here is the question that was asked. Please guide the student to the right answer. They won't be able to ask follow-up questions.",
            new_post_info.clone().title,
            new_post_info.clone().contents
        );
        // If AI response is requested get response and add reply
        let ai_response = match get_gemini_response(ai_input.clone()).await {
            Ok(response) => response,
            Err(e) => {
                error!("Failed to get AI response: {:?}", e);
                return Err(ServerFnError::ServerError("AI response failed".to_string()));
            }
        };

        if ai_response.is_empty() {
            error!("AI response is empty; check API or input formatting.");
            return Err(ServerFnError::ServerError("Empty AI response".to_string()));
        }

        let reply_info = AddReplyInfo {
            post_id: post.post_id,
            anonymous: false,
            contents: ai_response,
        };

        add_reply(reply_info, "EncampusAssistant".to_string()).await?;
    }
    Ok(post)
}

#[server(ResolvePost)]
pub async fn resolve_post(post_id: i32, status: bool) -> Result<(), ServerFnError> {
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
    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let posts: Vec<Post> = sqlx::query_as("select title, postid as post_id, resolved, private, authorid as author_id, endorsed, last_bumped, created_at from posts where to_tsvector(title || ' ' || contents) @@ to_tsquery($3) and classid = $1 and removed = false and ((posts.classid = $1 and private = false) or (posts.classid = $1 and authorid = $2 and private = true) or (classid = $1 and (select instructorid from classes where courseid = $1) = $2)) ORDER BY timestamp desc")
        .bind(class_id)
        .bind(user_id)
        .bind(filter_keyword)
        .fetch_all(&pool)
        .await
        .expect("No posts found");

    Ok(posts)
}

#[server(EditPost)]
pub async fn edit_post(
    post_id: i32,
    new_title: String,
    new_contents: String,
    user_id: i32,
    private: bool,
    anonymous: bool,
) -> Result<(), ServerFnError> {
    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let UserId(author_id) = sqlx::query_as("select authorid from posts where postid = $1")
        .bind(post_id)
        .fetch_one(&pool)
        .await
        .expect("Cannot get author id");

    if author_id == user_id {
        sqlx::query("update posts set title = $1, contents = $2, private = $3, anonymous = $4 where postid = $5")
            .bind(new_title)
            .bind(new_contents)
            .bind(private)
            .bind(anonymous)
            .bind(post_id)
            .execute(&pool)
            .await
            .expect("Cannot edit post");
    }
    Ok(())
}

#[server(EndorsePost)]
pub async fn endorse_post(post_id: i32, status: bool) -> Result<(), ServerFnError> {
    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    sqlx::query("update posts set endorsed = $1 where postid = $2")
        .bind(status)
        .bind(post_id)
        .execute(&pool)
        .await
        .expect("Cannot resolve post");

    Ok(())
}

#[server(BumpPost)]
pub async fn bump_post(post_id: i32) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete request".to_string(),
    ))?;

    sqlx::query("update posts set last_bumped = current_timestamp where postid = $1")
        .bind(post_id)
        .execute(&pool)
        .await
        .expect("Failed to bump post");

    Ok(())
}

#[server(GetTotalQuestions)]
pub async fn get_total_questions(class_id: i32) -> Result<i64, ServerFnError> {
    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM posts WHERE classid = $1 AND removed = false")
            .bind(class_id)
            .fetch_one(&pool)
            .await
            .expect("Failed to count questions");

    Ok(count.0)
}

#[server(GetResolvedQuestions)]
pub async fn get_resolved_questions(class_id: i32) -> Result<i64, ServerFnError> {
    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM posts WHERE classid = $1 AND resolved = true AND removed = false",
    )
    .bind(class_id)
    .fetch_one(&pool)
    .await
    .expect("Failed to count resolved questions");

    Ok(count.0)
}
// Used to get the amount of replies from ///
#[derive(Clone, Serialize, Deserialize, Default, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ReplyCounts {
    pub student_replies: i64,
    pub instructor_replies: i64,
}

// Used to get the amount of student and instructor replies for the question tile body.
#[server(GetReplyCounts)]
pub async fn get_reply_counts(post_id: i32) -> Result<ReplyCounts, ServerFnError> {
    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let counts: ReplyCounts = sqlx::query_as(
        r#"
        WITH filtered_replies AS (
            SELECT r.replyid,
                   CASE 
                       WHEN u.role IN ('instructor', 'ta') OR p.id IS NOT NULL THEN true 
                       ELSE false 
                   END as is_instructor
            FROM replies r
            JOIN users u ON r.authorid = u.id
            LEFT JOIN professors p ON u.id = p.id
            WHERE r.postid = $1 
            AND r.removed = false
        )
        SELECT 
            COUNT(CASE WHEN NOT is_instructor THEN 1 END) as student_replies,
            COUNT(CASE WHEN is_instructor THEN 1 END) as instructor_replies
        FROM filtered_replies
        "#,
    )
    .bind(post_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        ServerFnError::<NoCustomError>::ServerError(format!("Failed to get reply counts: {}", e))
    })?;

    Ok(counts)
}
