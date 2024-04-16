use leptos::{ev::MouseEvent, *};

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
