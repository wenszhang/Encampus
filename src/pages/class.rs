use leptos::{
    component, create_resource, server, view, For, IntoView, Params, ServerFnError, Suspense,
};
// use leptos::{component, view, For, IntoView, SignalWith};
use leptos_router::{use_params, Params};

use crate::components::header::Header;
use crate::components::question_tile::QuestionTile;

#[derive(Params, PartialEq, Clone)]
struct ClassId {
    class_id: i32,
}

/**
 * Page showing all questions in a class
 */
#[component]
pub fn ClassPage() -> impl IntoView {
    // Fetch params in the format of "class/:class_id"
    let class_id = use_params::<ClassId>();

    let titles = create_resource(class_id, |class_id| async {
        get_posts(class_id.unwrap().class_id)
            .await
            .unwrap_or_else(|_| vec!["Failed".to_string()])
    });

    let class_name = create_resource(class_id, |class_id| async {
        get_class_name(class_id.unwrap().class_id)
            .await
            .unwrap_or_else(|_| "Failed".to_string())
    });

    view! {
        <Suspense
            fallback=move || view! { <p>"Loading..."</p> }
            >
            <Header text={class_name().unwrap_or_default()} logo="logo.png".to_string() />
        </Suspense>

        <div class="grid grid-cols-3 gap-4 p-10 mx-20">
            <Suspense
                    fallback=move || view! { <p>"Loading..."</p> }
                >
                <For each=move || titles().unwrap_or_default() key=|post_title| post_title.clone() let:post_title>
                    <QuestionTile title={post_title} />
                </For>
            </Suspense>
        </div>
    }
}

#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow)]
struct Post(String);

#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow)]
struct ClassName(String);

#[server(GetPosts)]
async fn get_posts(class_id: i32) -> Result<Vec<String>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let rows: Vec<Post> = sqlx::query_as( "select title from posts join classes on posts.classid = classes.courseid where classes.courseid = $1")
        .bind(class_id)
        .fetch_all(&pool)
        .await
        .expect("select should work");

    let post_titles: Vec<String> = rows.into_iter().map(|row| row.0).collect();
    Ok(post_titles)
}

#[server(GetClassName)]
async fn get_class_name(class_id: i32) -> Result<String, ServerFnError> {
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
