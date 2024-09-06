use super::question_tile::QuestionTile;
use super::question_tile::UnansweredQuestionTile;
use crate::data::database::class_functions::get_class_name;
use crate::data::database::post_functions::get_posts;
use crate::data::database::post_functions::PostFetcher;
use crate::data::global_state::GlobalState;
use crate::pages::global_components::header::Header;
use leptos::*;
use leptos_router::{use_params, Outlet, Params, A};

#[derive(Params, PartialEq, Clone)]
pub struct ClassId {
    pub class_id: i32,
}

/**
 * Page getting and displaying all posts in a class
 */
#[component]
pub fn ClassPage() -> impl IntoView {
    let global_state = expect_context::<GlobalState>();
    // Fetch class id from route in the format of "class/:class_id"
    let class_id = use_params::<ClassId>();

    let post_data = PostFetcher {
        class_id: class_id.get().unwrap().class_id,
        user_id: global_state.id.get_untracked().unwrap(),
    };
    let posts = create_resource(
        move || (post_data),
        |post_data| async move {
            get_posts(post_data.class_id, post_data.user_id)
                .await
                .unwrap_or_default()
        },
    );
    provide_context(posts);

    let class_name = create_local_resource(class_id, |class_id| async {
        get_class_name(class_id.unwrap().class_id)
            .await
            .unwrap_or_else(|_| "Failed".to_string())
    });

    // TODO: Use signal to store the question title when clicking a tile
    let (question_title, _set_question_title) = create_signal("".to_string());

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
        <Suspense fallback=move || view! { <p>"Loading..."</p> } >
            <Header text={class_name().unwrap_or_default()} logo={None} class_id={Signal::derive(move || class_id().ok().map(|id| id.class_id))}/>
        </Suspense>
        <div class="flex justify-end pt-8 mx-20">
            <A href="new">
                <button class="bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">
                    "Post +"
                </button>
            </A>
        </div>

        <div class="flex align mx-20 my-10 flex-col gap-4">
            <Outlet/> // Gets replaced with the focused post if there's one in the route. See router

            <div class="grid grid-cols-3 gap-4">
                <Suspense fallback=move || view! { <p>"Loading..."</p> } >
                    <For each=move || posts().unwrap_or_default() key=|post| post.post_id let:post>
                        {post.resolved.then(|| view! { <QuestionTile post={post.clone()} />}).unwrap_or_else(|| view! { <UnansweredQuestionTile post={post.clone()} />})}
                    </For>
                </Suspense>
            </div>
        </div>
    }
}
