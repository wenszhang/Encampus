use leptos::{
    component, create_resource, server, view, For, IntoView, ServerFnError, SignalSet, Suspense,
};
//use leptos::{component, view, For, IntoView};

use crate::components::class_tile::ClassTile;
use crate::components::header::Header;

#[component]
fn ClassCard() -> impl IntoView {
    let class_name = create_resource(
        || {},
        |_| async move {
            let _class_names = get_class_name()
                .await
                .unwrap_or_else(|_err| vec!["Failed".to_string()]);
        },
    );

    view! {
        <div class="bg-white rounded shadow p-4 flex flex-col items-center justify-center font-bold h-32">
            <Suspense
                fallback=move || view! { <p>"Loading..."</p> }
            >
                {move || class_name().unwrap_or_default()}
            </Suspense>
            // <button class="border text-sm" on:click=move |_| { class_name.set("Loading...".to_string()); class_name.refetch(); }>
            //     "Click Me To Send Request to Server (watch the network requests)"
            // </button>
        </div>
    }
}

/**
 * Page showing all classes registered to the user
 */
#[component]
pub fn ClassesPage() -> impl IntoView {
    //TODO: Load this based on user data
    // let class_ids = vec![
    //     "Math3210".to_string(),
    //     "Class3124".to_string(),
    //     "Class4123".to_string(),
    //     "Class3214".to_string(),
    //     "Class1243".to_string(),
    // ];
    let class_ids = match get_class_name().await {
        Ok(names) => names,
        Err(err) => {
            vec!["Failed".to_string()]
        }
    };

    view! {
        <Header text="ENCAMPUS".to_string() logo="logo.png".to_string() />

        <div class="grid grid-cols-3 gap-4 p-10 mx-20">
            <For each=class_ids.iter().cloned() key=|class_id| class_id.clone()>
                <ClassTile class_id={class_id} />
            </For>
            // <For each=move || class_ids.clone() key=|id| id.clone() let:class_id>
            //     <ClassTile class_id={class_id} />
            // </For>
        </div>
    }
}

// struct Classname {
//     name: String,
// }

#[server(GetClassName)]
async fn get_class_name() -> Result<Vec<String>, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let row: Vec<(i32, String, String)> = sqlx::query_as("SELECT * from classes LIMIT 1")
        .fetch_all(&pool)
        .await
        .expect("select should work");

    let names: Vec<String> = row.into_iter().map(|(_, s1, _)| s1).collect();
    Ok(names)
}
