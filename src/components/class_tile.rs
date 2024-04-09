use leptos::*;

#[component]
pub fn ClassTile(title: String) -> impl IntoView {
    let onclick = move |_| {};

    view! {
        <div class="tile bg-white rounded shadow p-4 flex items-center justify-center font-bold h-32"
             on:click=onclick>
            {title}
        </div>
    }
}
