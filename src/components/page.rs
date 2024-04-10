use leptos::{component, view, IntoView};
use leptos_router::Outlet;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <div class="bg-gray-200 min-h-screen">
            <Outlet/>
        </div>
    }
}
