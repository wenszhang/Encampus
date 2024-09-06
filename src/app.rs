/**
 * This file is the main entry point for the application.
 */
use crate::{
    data::global_state::GlobalState,
    pages::{
        dev_pages::dev::Dev,
        global_components::{
            error_template::{AppError, ErrorTemplate},
            page::Page,
        },
        home::Home,
        login_page::LoginPage,
        view_class_posts::{class::ClassPage, create_post::CreatePost, focused_post::FocusedPost},
        view_enrolled_classes::classes::ClassesPage,
    },
};

use crate::pages::global_components::page::PageProps;
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
        <Title text="Encampus"/>

        // Routes
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

                    // Root route for Page, passing `show_sidebar` to control sidebar visibility
                    <Route path="/" view=move || Page(PageProps { show_sidebar: false })> // Hide sidebar by default
                        // Authenticated routes
                        <Route path="" view=AuthenticatedRoutes>
                            <Route path="/classes" view=ClassesPage/>
                            <Route path="/classes/:class_id" view=move || Page(PageProps { show_sidebar: true })> // Show sidebar on ClassPage
                                <Route path="" view=ClassPage/>
                                <Route path="new" view=CreatePost/>
                                <Route path=":post_id" view=FocusedPost/>
                            </Route>
                        </Route>

                        // Unauthenticated routes
                        <Route path="" view=UnauthenticatedRoutes>
                            <Route path="/" view=Home/>
                            <Route path="/login" view=LoginPage/>
                        </Route>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

/// Prevent errors due to loading pages that require authentication while logged out
#[component]
pub fn AuthenticatedRoutes() -> impl IntoView {
    let global_state = expect_context::<GlobalState>();

    let navigate = use_navigate();

    create_effect(move |_| {
        if !global_state.authenticated.get() {
            navigate("/login", Default::default());
        }
    });
    view! {
        <Outlet/>
    }
}

/// General route wrapping
#[component]
fn UnauthenticatedRoutes() -> impl IntoView {
    view! {
        <Outlet/>
    }
}
