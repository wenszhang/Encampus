use leptos::{
    component, create_resource, server, view, For, IntoView, ServerFnError, SignalWith, Suspense,
};
// use leptos::{component, view, For, IntoView, SignalWith};
use leptos_router::use_params_map;

use crate::components::header::Header;
use crate::components::question_tile::QuestionTile;

/**
 * Page showing all questions in a class
 */
#[component]
pub fn ClassPage() -> impl IntoView {
    // Fetch params in the format of "class/:class_id"
    let params = use_params_map();
    let class_id: String =
        params.with(|params| params.get("class_id").cloned().unwrap_or_default());

    // let class_name: String = class_id.clone();

    let titles = create_resource(
        || {},
        move |_| {
            let class_name = class_id.replace("-", " ");

            async move {
                get_posts(class_name.clone())
                    .await
                    .unwrap_or_else(|_| vec!["Failed".to_string()])
            }
        },
    );

    view! {
        <Header text={"class_id".to_string()} logo="logo.png".to_string() />

        <div class="grid grid-cols-3 gap-4 p-10 mx-20">
            <Suspense
                    fallback=move || view! { <p>"Loading..."</p> }
                >
                <For each=move || titles().unwrap_or_default().clone() key=|id| id.clone() let:class_id>
                    <QuestionTile title={class_id} />
                </For>
            </Suspense>
        </div>
    }
}

#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow)]
struct Post(String);

#[server(GetPosts)]
async fn get_posts(class_name: String) -> Result<Vec<String>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let query = format!(
        "select title from posts join classes on posts.classid = classes.courseid where classes.coursename = '{}'",
        "Math 3210"
    );

    let rows: Vec<Post> = sqlx::query_as(&query)
        .fetch_all(&pool)
        .await
        .expect("select should work");

    let post_titles: Vec<String> = rows.into_iter().map(|row| row.0).collect();
    Ok(post_titles)
}
