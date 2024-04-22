use leptos::{component, create_effect, leptos_dom, on_cleanup, view, IntoView};
use leptos_router::Outlet;

#[component]
pub fn Page() -> impl IntoView {
    create_effect(move |_| {
        leptos_dom::document().set_title("Encpamus");
        // Cleanup function (in case we ever need it)
        on_cleanup(move || {
            leptos_dom::document().set_title("Encpamus");
        });
    });

    view! {
        <div class="bg-gray-200 min-h-screen">
            <Outlet/>
        </div>
    }
}
