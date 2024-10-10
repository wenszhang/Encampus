/**
 * This file contains all the database functions that are used in the server
 */
use crate::data::database::user_functions::User;
use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ClassId(pub i32);

/**
 * Struct to hold the class info
 */
#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ClassInfo {
    pub id: i32,
    pub name: String,
    pub instructor_id: i32,
    pub instructor_name: String,
}

/**
 * Struct to hold the class name
 */
#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow)]
pub struct ClassName(String);

#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow)]
pub struct UserName(String);

/**
 * Get all class names from the database
 * Will eventually have a user added and so query will be modified to get only the classes the user is registered to
 */
#[server(GetClassList)]
pub async fn get_class_list() -> Result<Vec<ClassInfo>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let classes: Vec<ClassInfo> =
        sqlx::query_as("select classes.courseid as id, classes.coursename as name, instructing.professorid as instructor_id, CONCAT(users.firstname, ' ', users.lastname) as instructor_name 
        from classes join instructing on classes.courseid = instructing.courseid join users on instructing.professorid = users.id")
            .fetch_all(&pool)
            .await
            .expect("select should work");

    Ok(classes)
}

#[server(AddClass)]
pub async fn add_class(name: String, instructor: User) -> Result<ClassInfo, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let ClassId(class_id) = sqlx::query_as(
        "insert into classes (coursename, instructorid) values ($1, $2) returning courseid as id",
    )
    .bind(name.clone())
    .bind(instructor.id)
    .fetch_one(&pool)
    .await
    .expect("insert should work");

    Ok(ClassInfo {
        id: class_id,
        name,
        instructor_id: instructor.id,
        instructor_name: format!("{} {}", instructor.firstname, instructor.lastname),
    })
}

/**
 * Get the class name given the class id
 */
#[server(GetClassName)]
pub async fn get_class_name(class_id: i32) -> Result<String, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let ClassName(name) = sqlx::query_as("select coursename from classes where courseid = $1")
        .bind(class_id)
        .fetch_one(&pool)
        .await
        .expect("select should work");
    Ok(name)
}

#[server(GetInstructor)]
pub async fn get_instructor(post_id: i32) -> Result<String, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let UserName (name)= sqlx::query_as("select username from classes join users on instructorid = id where courseid = (select classid from posts where postid = $1)")
    .bind(post_id)
    .fetch_one(&pool)
    .await
    .expect("select should work");
    Ok(name)
}

#[server(AddStudentToClass)]
pub async fn add_student_to_class(class_id: i32, user_id: i32) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    sqlx::query("insert into enrolled (studentid, courseid) values ($1, $2)")
        .bind(user_id)
        .bind(class_id)
        .execute(&pool)
        .await
        .expect("Failed adding user to class");
    Ok(())
}

#[server(RemoveStudentFromClass)]
pub async fn remove_student_from_class(class_id: i32, user_id: i32) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    sqlx::query("delete from enrolled where courseid = $1 and studentid = $2")
        .bind(class_id)
        .bind(user_id)
        .execute(&pool)
        .await
        .expect("Failed adding user to class");
    Ok(())
}

#[server(GetStudentsClasses)]
pub async fn get_students_classes(user_id: i32) -> Result<Vec<ClassInfo>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let classes: Vec<ClassInfo> = sqlx::query_as("select classes.courseid as id, classes.coursename as name, CONCAT(users.firstname, ' ', users.lastname) as instructor 
    from classes join instructing on classes.courseid = instructing.courseid join users on instructing.professorid = users.id join enrolled on classes.courseid = enrolled.courseid where enrolled.studentid = $1")
        .bind(user_id)
        .fetch_all(&pool)
        .await
        .expect("select should work");

    Ok(classes)
}
