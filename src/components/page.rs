use leptos::{component, view, IntoView};
use leptos_meta::Stylesheet;
use leptos_router::Outlet;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/encampus.css"/>

        <div class="bg-gray-200 min-h-screen">
            <Outlet/>
        </div>
    }
}
