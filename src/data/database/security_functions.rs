#[allow(unused_imports)] // Suppress User - false compiler warning due to RowToStruct
use super::user_functions::User;
use leptos::{server, ServerFnError};

/**
 * Login a user or sign them up if they don't exist
 */
#[server(LoginSignUp)]
pub async fn login_signup(username: String) -> Result<i32, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let mut user_result: Option<User> = sqlx::query_as(
        "select username, first_name, last_name, id from users where username = $1
                                RETURNING id;",
    )
    .bind(username.clone())
    .fetch_optional(&pool)
    .await?;

    if user_result.is_none() {
        sqlx::query("insert into users(username) values($1)")
            .bind(username.clone())
            .execute(&pool)
            .await
            .expect("Failed adding user");

        sqlx::query("insert into students(name) values($1)")
            .bind(username.clone())
            .execute(&pool)
            .await
            .expect("Failed adding student");

        user_result = sqlx::query_as(
            "select username, first_name, last_name, id from users where username = $1
                                    RETURNING id;",
        )
        .bind(username.clone())
        .fetch_optional(&pool)
        .await?;
    }

    Ok(user_result.unwrap().id)
}
