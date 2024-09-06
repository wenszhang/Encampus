/**
 * Page 'framework' that wraps all pages and provides a sidebar
 */
use crate::pages::global_components::sidebar::Sidebar;
use leptos::SignalGet;
use leptos::{component, view, IntoView, MaybeSignal};
use leptos_meta::Title;
use leptos_router::Outlet;

#[component]
pub fn Page(show_sidebar: bool) -> impl IntoView {
    view! {
        <Title text="Encampus"/>
        <div class="flex flex-col bg-gray-200 min-h-screen">
            {move || if show_sidebar {
                view! {
                    <div class="flex">
                        <div class="fixed w-64">
                            <Sidebar/>
                        </div>
                        <div class="flex-1 ml-64">
                            <Outlet/>
                        </div>
                    </div>
                }.into_view()
            } else {
                view! {
                    <div class="flex">
                        <div class="flex-1">
                            <Outlet/>
                        </div>
                    </div>
                }.into_view()
            }}
        </div>
    }
}
