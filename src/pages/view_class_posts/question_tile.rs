/**
 * QuestionTile component, displaying a tile for one post
 */
use crate::data::database::post_functions::Post;
use leptos::*;
use leptos_router::A;

#[component]
pub fn QuestionTile(post: Post) -> impl IntoView {
    view! {
        <A href=format!("{}", post.post_id)>
            <div class="relative bg-white rounded-lg shadow-lg p-6 flex flex-col items-center justify-center text-lg font-semibold h-60 w-85 transition-transform duration-300 hover:scale-105 hover:bg-gray-100 hover:shadow-xl overflow-hidden">
                {post.title}
            </div>
        </A>
    }
}

#[component]
pub fn UnansweredQuestionTile(post: Post) -> impl IntoView {
    view! {
        <A href=format!("{}", post.post_id)>
            <div class="relative bg-red-500 hover:bg-red-700 rounded-lg shadow-lg p-6 flex flex-col items-center justify-center text-lg font-semibold h-60 w-100 transition-transform duration-300 hover:scale-105hover:shadow-xl overflow-hidden">
                {post.title}
            </div>
        </A>
    }
}
