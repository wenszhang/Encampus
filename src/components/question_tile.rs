/**
 * QuestionTile component, displaying a tile for one post
 */
use crate::database_functions::Post;
use leptos::*;
use leptos_router::A;

#[component]
pub fn QuestionTile(post: Post) -> impl IntoView {
    view! {
        <A href=format!("{}", post.post_id)>
            <div class="tile bg-white rounded shadow p-4 flex items-center justify-center font-bold h-32 transition duration-300 hover:bg-gray-100">
                {post.title}
            </div>
        </A>
    }
}
