use leptos::ev::MouseEvent;
use leptos::{component, create_signal, view, CollectView, For, IntoView, Show, SignalWith};
use leptos_router::use_params_map;

use crate::components::header::Header;

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

    let (is_modal_open, set_modal_open) = create_signal(false);

    view! {
        <Header text={class_id.clone()} logo="logo.png".to_string() />

        <div class="mx-20">
            <Show when=is_modal_open>
                <QuestionDisplay class_id=class_id.clone() question_id="1".to_string()/>
            </Show>

            <div class="grid grid-cols-3 p-t-10 gap-4">
                <For each=move || titles.clone() key=|title| title.clone() let:class_id>
                    <QuestionTile title={class_id.clone()} on_click_handler=move |_| set_modal_open(true)/>
                </For>
            </div>
        </div>
    }
}

#[component]
pub fn QuestionTile<F>(title: String, on_click_handler: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! {
        <div class="tile bg-white rounded shadow p-4 flex items-center justify-center font-bold h-32"
            on:click = on_click_handler>
            {title}
        </div>
    }
}

struct Reply {
    pub author: String,
    pub reply: String,
}

#[component]
pub fn QuestionDisplay(class_id: String, question_id: String) -> impl IntoView {
    // Get info...
    let question_title = class_id + " Question " + &question_id;
    let responses = vec![
        Reply {author: "Alice".to_string(), reply: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string()},
        Reply {author: "Bob".to_string(), reply: "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.".to_string()},
        Reply {author: "Charlie".to_string(), reply: "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.".to_string()},
    ];

    let replies_list = responses
        .into_iter()
        .map(|response| {
            view! {
                <SingleReplyBox author=response.author.clone() reply=response.reply.clone()/>
            }
        })
        .collect_view();

    view! {
        <div class="mx-auto p-4">
            // Column container
            <div class="space-y-4">
                // Question box
                <div class="bg-white shadow rounded-lg p-6">
                    <h2 class="text-lg font-semibold mb-2">{question_title}</h2>
                    <p class="text-gray-600">Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.</p>
                </div>
                // Replies
                <div>{replies_list}</div>
            </div>
        </div>
    }
}

#[component]
pub fn SingleReplyBox(author: String, reply: String) -> impl IntoView {
    view! {
        <div class="bg-white shadow rounded-lg p-4">
            <h2 class="text-lg font-semibold mb-2">{author}</h2>
            <p class="text-gray-600">{reply}</p>
        </div>
    }
}
