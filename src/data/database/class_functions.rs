/**
 * This file contains all the database functions that are used in the server
 */
use crate::data::database::user_functions::DbUser;
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
    pub description: String,
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
 * Struct to hold enrollment info
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct UserEnrollmentInfo {
    pub user_id: i32,
    pub full_name: String,
    pub role: String,
}

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
        sqlx::query_as("select classes.courseid as id, classes.coursename as name, instructing.professorid as instructor_id, CONCAT(users.firstname, ' ', users.lastname) as instructor_name, description 
        from classes join instructing on classes.courseid = instructing.courseid join users on instructing.professorid = users.id")
            .fetch_all(&pool)
            .await
            .expect("select should work");

    Ok(classes)
}

#[server(AddClass)]
pub async fn add_class(
    name: String,
    instructor_username: i32,
    class_description: String,
) -> Result<ClassInfo, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let instructor: DbUser =
        sqlx::query_as("select username, firstname, lastname, id, role from users where id = $1")
            .bind(instructor_username)
            .fetch_one(&pool)
            .await
            .expect("Failed getting instructor");

    let ClassId(class_id) = sqlx::query_as(
        "insert into classes (coursename, instructorid, coursesection, description) values ($1, $2, 90, $3) returning courseid as id",
    )
    .bind(name.clone())
    .bind(instructor.id)
    .bind(class_description.clone())
    .fetch_one(&pool)
    .await
    .expect("Failed adding class");

    sqlx::query("insert into instructing (professorid, courseid) values ($1, $2)")
        .bind(instructor.id)
        .bind(class_id)
        .execute(&pool)
        .await
        .expect("Failed adding instructor to class");

    // Add Encampus Assistant to the class, id is 334 for Encampus Assistant
    sqlx::query("insert into ta (id, classid) values ($1, $2)")
        .bind(334)
        .bind(class_id)
        .execute(&pool)
        .await
        .expect("Failed adding Encampus Assistant");

    Ok(ClassInfo {
        id: class_id,
        name,
        instructor_id: instructor.id,
        instructor_name: format!("{} {}", instructor.firstname, instructor.lastname),
        description: class_description.clone(),
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

    let classes: Vec<ClassInfo> = sqlx::query_as("select classes.courseid as id, classes.coursename as name, instructing.professorid as instructor_id, CONCAT(users.firstname, ' ', users.lastname) as instructor_name, description 
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
        let classes: Vec<ClassInfo> = sqlx::query_as("select classes.courseid as id, classes.coursename as name, instructing.professorid as instructor_id, CONCAT(users.firstname, ' ', users.lastname) as instructor_name, description 
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

    sqlx::query("update classes set coursename = $1, instructorid = $2, description = $4 where courseid = $3")
        .bind(class.name)
        .bind(instructor_id)
        .bind(class.id)
        .bind(class.description)
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

#[server(AddTAToClass)]
pub async fn add_ta_to_class(user_id: i32, class_id: i32) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    sqlx::query("insert into ta (id, classid) values ($1, $2)")
        .bind(user_id)
        .bind(class_id)
        .execute(&pool)
        .await
        .expect("Failed adding user to class");
    Ok(())
}

#[server(RemoveTAFromClass)]
pub async fn remove_ta_from_class(user_id: i32, class_id: i32) -> Result<(), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    sqlx::query("delete from ta where id = $1 and classid = $2")
        .bind(user_id)
        .bind(class_id)
        .execute(&pool)
        .await
        .expect("Failed adding user to class");
    Ok(())
}

#[server(GetClassesTA)]
pub async fn get_classes_ta(user_id: i32) -> Result<Vec<ClassInfo>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let classes: Vec<ClassInfo> = sqlx::query_as("select ta.classid as id, classes.coursename as name, instructing.professorid as instructor_id, CONCAT(users.firstname, ' ', users.lastname) as instructor_name 
        from ta join classes on ta.classid = classes.courseid join instructing on classes.courseid = instructing.courseid join users on instructing.professorid = users.id where ta.id = $1")
        .bind(user_id)
        .fetch_all(&pool)
        .await
        .expect("select should work");

    Ok(classes)
}

#[server(GetClassDescription)]
pub async fn get_class_description(class_id: i32) -> Result<String, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;
    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;
    let ClassName(description) =
        sqlx::query_as("select description from classes where courseid = $1")
            .bind(class_id)
            .fetch_one(&pool)
            .await
            .expect("Unable to get description");
    Ok(description)
}

#[server(GetUsersEnrolledInClass)]
pub async fn get_users_enrolled_in_class(
    class_id: i32,
) -> Result<Vec<UserEnrollmentInfo>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let users: Vec<UserEnrollmentInfo> = sqlx::query_as(
        "SELECT
            u.id as user_id,
            CONCAT(u.firstname, ' ', u.lastname) as full_name,
            CASE
                WHEN i.professorid IS NOT NULL THEN 'Instructor'
                WHEN t.id IS NOT NULL THEN 'TA'
                WHEN e.studentid IS NOT NULL THEN 'Student'
                ELSE 'Unknown'
            END as role
        FROM users u
        LEFT JOIN instructing i ON u.id = i.professorid AND i.courseid = $1
        LEFT JOIN ta t ON u.id = t.id AND t.classid = $1
        LEFT JOIN enrolled e ON u.id = e.studentid AND e.courseid = $1
        WHERE i.professorid IS NOT NULL
            OR t.id IS NOT NULL
            OR e.studentid IS NOT NULL
        ",
    )
    .bind(class_id)
    .fetch_all(&pool)
    .await
    .expect("Failed to retrieve enrolled users with roles");

    Ok(users)
}
