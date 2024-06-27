use leptos::{component, create_signal, view, IntoView, SignalUpdate};
use leptos_router::A;

#[component]
pub fn Dev() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>

        <div class="bg-gray-200 p-4 rounded-md shadow-md hover:shadow-lg text-gray-700 max-w-md mx-auto">
            <h2 class="font-bold text-xl pb-2">"Tailwind Test Delete later"</h2>
        </div>

        <A href="/classes">"Classes Page"</A>
    }
}
