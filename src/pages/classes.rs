use leptos::{
    component, create_resource, server, view, CollectView, For, IntoView, ServerFnError, SignalSet,
    Suspense,
};
//use leptos::{component, view, For, IntoView};

//use crate::components::class_tile::ClassTile;
use crate::components::header::Header;

#[component]
pub fn ClassTile(class_id: String) -> impl IntoView {
    view! {
        <a href=&format!("class/{}", class_id.replace(" ", ""))>
        <div class="tile bg-white rounded shadow p-4 flex items-center justify-center font-bold h-32">
            {class_id} //TODO: get title from DB
        </div>
        </a>
    }
}

// #[component]
// fn ClassCard() -> impl IntoView {
//     let class_names = create_resource(
//         || {},
//         |_| async move { get_class_name().await.unwrap_or(vec!["Failed".to_string()]) },
//     );

//     let classes_list = class_names()
//         .unwrap_or_default()
//         .into_iter()
//         .map(|class| {
//             view! {
//                     <ClassTile class_id=class.clone()/>
//             }
//         })
//         .collect_view();
// }

/**
 * Page showing all classes registered to the user
 */
#[component]
pub fn ClassesPage() -> impl IntoView {
    let class_names = create_resource(
        || {},
        |_| async move { get_class_name().await.unwrap_or(vec!["Failed".to_string()]) },
    );

    // let classes_list = class_names()
    //     .unwrap_or_default()
    //     .into_iter()
    //     .map(|class| {
    //         view! {
    //             {println!("class names: {:?}", class_names());}
    //             <ClassTile class_id=class.clone()/>
    //         }
    //     })
    //     .collect_view();

    view! {
        <Header text="ENCAMPUS".to_string() logo="logo.png".to_string() />

        <div class="grid grid-cols-3 gap-4 p-10 mx-20">
            <For each=move || class_names().unwrap_or_default().clone() key=|id| id.clone() let:class_id>
                <ClassTile class_id={class_id} />
            </For>

           //{classes_list}
        </div>
    }
}

#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow)]
struct Classname(String);

#[cfg(feature = "ssr")]
#[derive(sqlx::FromRow)]
struct ClassNumPosts(i32);

#[server(GetClassName)]
async fn get_class_name() -> Result<Vec<String>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let rows: Vec<Classname> = sqlx::query_as("SELECT coursename from classes")
        .fetch_all(&pool)
        .await
        .expect("select should work");

    let names: Vec<String> = rows.into_iter().map(|row| row.0).collect();
    Ok(names)
}

// #[server(GetNumClasses)]
// async fn get_num_classes() -> Result<i32, ServerFnError> {
//     use leptos::{server_fn::error::NoCustomError, use_context};
//     use sqlx::postgres::PgPool;

//     let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
//         "Unable to complete Request".to_string(),
//     ))?;

//     let ClassNumPosts(num_posts) = sqlx::query_as("SELECT COUNT(*) FROM classes;")
//         .fetch_one(&pool)
//         .await
//         .expect("select should work");
//     Ok(num_posts)
// }
