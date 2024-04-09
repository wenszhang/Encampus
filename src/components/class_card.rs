use leptos::*;

/// component for class cards
#[component]
pub fn ClassCard(
    /// currently takes a string
    /// later on should take a uid and return a list
    class_name: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded shadow p-4 flex items-center justify-center font-bold h-32">
            {class_name}
        </div>
    }
}
