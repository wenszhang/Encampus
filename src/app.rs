use crate::{
    components::{focused_post::FocusedPost, page::Page},
    error_template::{AppError, ErrorTemplate},
    pages::{class::ClassPage, classes::ClassesPage, dev::Dev, home::Home, login_page::LoginPage},
    util::global_state::GlobalState,
};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_context(GlobalState::new());

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
                    <Route path="/dev" view=Dev/>
                    <Route path="" view=Page>
                        <Route path="" view=Home/>
                        <Route path="/classes" view=ClassesPage/>
                        <Route path="/classes/:class_id" view=ClassPage>
                            <Route path="" view=|| {}/>
                            // <Route path="/new" view=CreatePost/>
                            <Route path="/:post_id" view=FocusedPost/>
                        </Route>
                        <Route path="/login" view=LoginPage/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}
