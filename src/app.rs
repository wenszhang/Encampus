/**
 * This file is the main entry point for the application.
 */
use crate::{
    data::global_state::GlobalState,
    pages::{
        admin_pages::admin_homepage::AdminHomePage,
        dev_pages::dev::Dev,
        global_components::{
            error_template::{AppError, ErrorTemplate},
            page::Page,
        },
        home::Home,
        login_page::LoginPage,
        register_page::RegisterPage,
        user_profile::user_profile_page::UserProfile,
        user_settings::user_settings_page::UserSettings,
        view_class_posts::{
            announcement_details::AnnouncementDetails, class::ClassPage,
            class_details::ClassDetails, focused_post::FocusedPost,
        },
        view_enrolled_classes::classes::ClassesPage,
    },
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
      {}
      // added this empty {} to prevent lint from removing these comments

      // injects a stylesheet into the document <head>
      // id=leptos means cargo-leptos will hot-reload this stylesheet
      <Stylesheet id="leptos" href="/pkg/encampus.css" />

      // sets the document title
      <Title text="Encampus" />

      // Routes
      <Router fallback=|| {
        let mut outside_errors = Errors::default();
        outside_errors.insert_with_default_key(AppError::NotFound);
        view! { <ErrorTemplate outside_errors /> }.into_view()
      }>
        <main>
          <Routes>
            <Route path="/dev" view=Dev />
            <Route path="" view=Page>
              // Only accessible when logged in
              <Route path="" view=AuthenticatedRoutes>
                <Route path="/AdminHomePage" view=AdminHomePage />
                <Route path="/classes" view=ClassesPage />
                <Route path="/classes/:class_id" view=ClassPage>
                  <Route path="" view=|| {} />
                  <Route path="/details" view=ClassDetails />
                  <Route path="/:post_id" view=FocusedPost />
                  <Route path="/announcement/:announcement_id" view=AnnouncementDetails />
                </Route>
                <Route path="/settings" view=UserSettings />
                <Route path="/profile" view=UserProfile />
              </Route>
              // Accessible when logged out
              <Route path="" view=UnauthenticatedRoutes>
                <Route path="" view=Home />
                <Route path="/login" view=LoginPage />
                <Route path="/register" view=RegisterPage />
              </Route>
            </Route>
          </Routes>
        </main>
      </Router>
    }
}

/// Prevent errors due to loading pages that require authentication while logged
/// out
#[component]
pub fn AuthenticatedRoutes() -> impl IntoView {
    let global_state = expect_context::<GlobalState>();

    let navigate = use_navigate();

    create_effect(move |_| {
        if !global_state.authenticated.get() {
            navigate("/login", Default::default());
        }
    });
    view! { <Outlet /> }
}

/// General route wrapping
#[component]
fn UnauthenticatedRoutes() -> impl IntoView {
    view! { <Outlet /> }
}
