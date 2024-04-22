use leptos::leptos_dom;
use leptos::{
    component, create_effect, create_resource, create_signal, view, For, IntoView, Params, Suspense,
};
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

    // TODO: Use signal to store the question title when clicking a tile
    let (question_title, set_question_title) = create_signal("".to_string());

    // Reactively update the document title when class_name or question_title changes.
    create_effect(move |_| {
        let current_class_name = class_name().unwrap_or_else(|| "Unknown Class".to_string());
        // Only show question title if it is not empty
        let title = if question_title().is_empty() {
            current_class_name
        } else {
            format!("{} - {}", current_class_name, question_title())
        };
        leptos_dom::document().set_title(&title);
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
