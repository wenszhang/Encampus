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
#[derive(Clone, Serialize, Deserialize, Default, Debug)]
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
pub async fn add_class(name: String, instructor_username: i32) -> Result<ClassInfo, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let instructor: User =
        sqlx::query_as("select username, firstname, lastname, id, role from users where id = $1")
            .bind(instructor_username)
            .fetch_one(&pool)
            .await
            .expect("Failed getting instructor");

    let ClassId(class_id) = sqlx::query_as(
        "insert into classes (coursename, instructorid, coursesection) values ($1, $2, 90) returning courseid as id",
    )
    .bind(name.clone())
    .bind(instructor.id)
    .fetch_one(&pool)
    .await
    .expect("Failed adding class");

    sqlx::query("insert into instructing (professorid, courseid) values ($1, $2)")
        .bind(instructor.id)
        .bind(class_id)
        .execute(&pool)
        .await
        .expect("Failed adding instructor to class");

    Ok(ClassInfo {
        id: class_id,
        name,
        instructor_id: instructor.id,
        instructor_name: format!("{} {}", instructor.firstname, instructor.lastname),
    })
}

#[server(DeleteClass)]
pub async fn delete_class(class_id: i32) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    sqlx::query("delete from instructing where courseid = $1")
        .bind(class_id)
        .execute(&pool)
        .await
        .expect("Failed deleting instructor from class");
    sqlx::query("delete from enrolled where courseid = $1")
        .bind(class_id)
        .execute(&pool)
        .await
        .expect("Failed deleting enrolled students from class");
    sqlx::query("delete from classes where courseid = $1")
        .bind(class_id)
        .execute(&pool)
        .await
        .expect("Failed deleting class");

    Ok(())
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

    let classes: Vec<ClassInfo> = sqlx::query_as("select classes.courseid as id, classes.coursename as name, instructing.professorid as instructor_id, CONCAT(users.firstname, ' ', users.lastname) as instructor_name 
    from classes join instructing on classes.courseid = instructing.courseid join users on instructing.professorid = users.id join enrolled on classes.courseid = enrolled.courseid where enrolled.studentid = $1")
        .bind(user_id)
        .fetch_all(&pool)
        .await
        .expect("select should work");

    Ok(classes)
}

#[server(GetInstructorsClasses)]
pub async fn get_instructors_classes(user_id: i32) -> Result<Vec<ClassInfo>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let classes: Vec<ClassInfo> = sqlx::query_as("select classes.courseid as id, classes.coursename as name, instructing.professorid as instructor_id, CONCAT(users.firstname, ' ', users.lastname) as instructor_name 
    from classes join instructing on classes.courseid = instructing.courseid join users on instructing.professorid = users.id where instructing.professorid = $1")
        .bind(user_id)
        .fetch_all(&pool)
        .await
        .expect("select should work");

    Ok(classes)
}

#[server(GetUsersClasses)]
pub async fn get_users_classes(
    user_id: i32,
    role: String,
) -> Result<Vec<ClassInfo>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    if role == "Student" {
        let classes: Vec<ClassInfo> = sqlx::query_as("select classes.courseid as id, classes.coursename as name, instructing.professorid as instructor_id, CONCAT(users.firstname, ' ', users.lastname) as instructor_name 
        from classes join instructing on classes.courseid = instructing.courseid join users on instructing.professorid = users.id join enrolled on classes.courseid = enrolled.courseid where enrolled.studentid = $1")
            .bind(user_id)
            .fetch_all(&pool)
            .await
            .expect("select should work");

        Ok(classes)
    } else if role == "Instructor" {
        let classes: Vec<ClassInfo> = sqlx::query_as("select classes.courseid as id, classes.coursename as name, instructing.professorid as instructor_id, CONCAT(users.firstname, ' ', users.lastname) as instructor_name 
        from classes join instructing on classes.courseid = instructing.courseid join users on instructing.professorid = users.id where instructing.professorid = $1")
            .bind(user_id)
            .fetch_all(&pool)
            .await
            .expect("select should work");

        Ok(classes)
    } else {
        return Err(ServerFnError::<NoCustomError>::ServerError(
            "Invalid role".to_string(),
        ));
    }
}

#[server(UpdateClassInfo)]
pub async fn update_class_info(class: ClassInfo, instructor_id: i32) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    sqlx::query("update classes set coursename = $1, instructorid = $2 where courseid = $3")
        .bind(class.name)
        .bind(instructor_id)
        .bind(class.id)
        .execute(&pool)
        .await
        .map_err(|_| {
            ServerFnError::<NoCustomError>::ServerError("Unable to update user".to_string())
        })?;

    sqlx::query("update instructing set professorid = $1 where courseid = $2")
        .bind(instructor_id)
        .bind(class.id)
        .execute(&pool)
        .await
        .map_err(|_| {
            ServerFnError::<NoCustomError>::ServerError("Unable to update user".to_string())
        })?;
    Ok(())
}

#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow)]
pub struct IsInstructor(i64);

#[server(CheckUserIsInstructor)]
pub async fn check_user_is_instructor(user_id: i32, class_id: i32) -> Result<bool, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let IsInstructor(instructor_count) = sqlx::query_as(" select count(*) from instructing join ta on instructing.courseid = ta.classid where 
                                                    (ta.id = $1 or instructing.professorid = $1) and ta.classid = $2")
        .bind(user_id)
        .bind(class_id)
        .fetch_one(&pool)
        .await
        .expect("select should work");

    if instructor_count > 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}
