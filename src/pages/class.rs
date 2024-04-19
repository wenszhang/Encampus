use leptos::{component, create_resource, view, For, IntoView, Params, Suspense};
// use leptos::{component, view, For, IntoView, SignalWith};
use leptos_router::{use_params, Params};

use crate::components::header::Header;
use crate::components::question_tile::QuestionTile;
use crate::database_functions::get_class_name;
use crate::database_functions::get_posts;

#[derive(Params, PartialEq, Clone)]
pub struct ClassId {
    pub class_id: i32,
}

#[derive(Params, PartialEq)]
struct ClassParams {
    class_id: String,
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
            <Header text={class_name().unwrap_or_default()} logo="logo.png".to_string() user="LONGNAME".to_string()/>
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
