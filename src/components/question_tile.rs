use leptos::*;

#[component]
pub fn QuestionTile(title: String) -> impl IntoView {
    view! {
        <div class="tile bg-white rounded shadow p-4 flex items-center justify-center font-bold h-32">
            {title}
        </div>
    }
}
