#[allow(unused_imports)] // Suppress User - false compiler warning due to RowToStruct
use super::user_functions::User;
use leptos::{server, ServerFnError};

/**
 * Login a user or sign them up if they don't exist
 */
#[server(LoginSignUp)]
pub async fn login_signup(name: String) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let user_result: Option<User> = sqlx::query_as("select name, id from users where name = $1")
        .bind(name.clone())
        .fetch_optional(&pool)
        .await?;

    if user_result.is_none() {
        sqlx::query("insert into users(name) values($1)")
            .bind(name.clone())
            .execute(&pool)
            .await
            .expect("Failed adding user");

        sqlx::query("insert into students(name) values($1)")
            .bind(name.clone())
            .execute(&pool)
            .await
            .expect("Failed adding student");
    }

    Ok(())
}
