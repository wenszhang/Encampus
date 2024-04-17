use leptos::{component, create_resource, server, view, For, IntoView, ServerFnError, Suspense};
use serde::{Deserialize, Serialize};

//use crate::components::class_tile::ClassTile;
use crate::components::header::Header;

#[component]
pub fn ClassTile(class: ClassInfo) -> impl IntoView {
    view! {
        <a href=&format!("class/{}", class.id)>
            <div class="tile bg-white rounded shadow p-4 flex items-center justify-center font-bold h-32">
                {class.name}
            </div>
        </a>
    }
}

/**
 * Page showing all classes registered to the user
 */
#[component]
pub fn ClassesPage() -> impl IntoView {
    let classes = create_resource(|| {}, |_| async { get_class_list().await.unwrap() });

    // let classes_list = class_names()
    //     .clone()
    //     .unwrap_or_default()
    //     .into_iter()
    //     .map(|class| {
    //         view! {
    //             <ClassTile class_id=class.clone()/>
    //         }
    //     })
    //     .collect_view();

    view! {
        <Header text="ENCAMPUS".to_string() logo="logo.png".to_string() />

        <div class="grid grid-cols-3 gap-4 p-10 mx-20">
            <Suspense
                fallback=move || view! { <p>"Loading..."</p> }
            >
                <For each=move || classes().unwrap_or_default() key=|class| class.id let:class>
                    <ClassTile class={class} />
                </For>

                // {classes_list}
            </Suspense>
        </div>
    }
}

/**
 * Struct to hold the class info
 */

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ClassInfo {
    id: i32,
    name: String,
}

/**
 * Get all class names from the database
 * Will eventually have a user added and so query will be modified to get only the classes the user is registered to
 */
#[server(GetClassName)]
async fn get_class_list() -> Result<Vec<ClassInfo>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let classes: Vec<ClassInfo> =
        sqlx::query_as("SELECT courseid as id, coursename as name from classes")
            .fetch_all(&pool)
            .await
            .expect("select should work");

    Ok(classes)
}
