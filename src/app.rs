use crate::{
    components::page::Page,
    error_template::{AppError, ErrorTemplate},
    pages::class::ClassPage,
    pages::classes::ClassesPage,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/encampus.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="/dev" view=DevPage/>
                    <Route path="" view=Page>
                        <Route path="" view=HomePage/>
                        <Route path="/classes" view=ClassesPage/>
                        <Route path="/class/:class_id" view=ClassPage/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <A href="/dev">"Dev Page"</A>
        <p></p>
        <A href="/classes">"Classes Page"</A>
    }
}

#[component]
fn DevPage() -> impl IntoView {
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
