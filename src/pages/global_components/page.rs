/**
 * Page component
 */
use leptos::{component, view, IntoView};
use leptos_meta::Title;
use leptos_router::Outlet;

#[component]
pub fn Page() -> impl IntoView {
    view! {
      <Title text="Encampus" />
      <div class="flex flex-col min-h-screen bg-gray-200">
        <Outlet />
      </div>
    }
}
