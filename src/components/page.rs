use leptos::{component, view, IntoView};
use leptos_meta::Title;
use leptos_router::Outlet;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <Title text="Encapmus"/>
        <div class="bg-gray-200 min-h-screen">
            <Outlet/>
        </div>
    }
}
