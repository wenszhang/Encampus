use leptos::{component, view, For, IntoView, SignalWith};
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

    // Dummy data
    let titles = vec![
        "Question 1".to_string(),
        "Question 2".to_string(),
        "Question 3".to_string(),
        "Question 4".to_string(),
        "Question 5".to_string(),
    ];

    view! {
        <Header text={class_id} logo="logo.png".to_string() />

        <div class="grid grid-cols-3 gap-4 p-10 mx-20">
            <For each=move || titles.clone() key=|id| id.clone() let:class_id>
                <QuestionTile title={class_id} />
            </For>
        </div>
    }
}
