use leptos::*;

#[component]
pub fn ClassTile(class_id: String) -> impl IntoView {
    view! {
        <a href=&format!("class/{}", class_id)>
        <div class="tile bg-white rounded shadow p-4 flex items-center justify-center font-bold h-32">
            {class_id} //TODO: get title from DB
        </div>
        </a>
    }
}
