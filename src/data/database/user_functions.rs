/**
 * Struct to hold user
 */
#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow)]
pub struct User {
    pub name: String,
    pub id: i32,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct UserId(pub i32);
