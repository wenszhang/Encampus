/**
 * This file is the main entry point for the application.
 */
use crate::{
    data::global_state::{AuthContext, Authentication},
    pages::{
        about_us_pages::{about_us::AboutUs, tutorial::Tutorial},
        admin_pages::admin_homepage::AdminHomePage,
        global_components::{
            error_template::{AppError, ErrorTemplate},
            page::Page,
        },
        home::Home,
        live_poll::LivePoll,
        login_page::LoginPage,
        register_page::RegisterPage,
        user_profile::user_profile_page::UserProfile,
        user_settings::user_settings_page::UserSettings,
        view_class_posts::{
            announcement_details::AnnouncementDetails, class::ClassPage,
            class_details::ClassDetails, edit_post::EditPost, focused_post::FocusedPost,
        },
        view_enrolled_classes::classes::ClassesPage,
    },
};
use leptoaster::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

const EMBEDDED_AUTHENTICATION_KEY: &str = "__EMBEDDED_ENCAMPUS_AUTHENTICATION__";

#[component]
pub fn App() -> impl IntoView {
    // creating a toast through a leptos toast library.
    provide_toaster();

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Checks for auth once the front end starts to hydrate. Only runs in the browser.
    provide_authentication_from_window_context();

    // Expects the auth context to be provided.
    // This is because in the browser we provide the auth context from the window object and in the server we provide it in middleware.
    let auth = expect_auth_context();

    view! {
      {auth
        .get_untracked()
        .get_user()
        .and_then(|user| {
          let user_str = serde_json::to_string(user).ok()?;
          Some(
            view! {
              <Script>{format!("window.{} = `{}`;", EMBEDDED_AUTHENTICATION_KEY, user_str)}</Script>
            },
          )
        })}

      // tiptap.js files
      <Script type_="module" src="/js/tiptap-bundle.min.js" />
      <Script type_="module" src="/js/tiptap.js" />

      // injects a stylesheet into the document <head>
      // id=leptos means cargo-leptos will hot-reload this stylesheet
      <Stylesheet id="leptos" href="/pkg/encampus.css" />

      // sets the document title
      <Title text="Encampus" />

      // Provide toaster
      <Toaster />

      // Routes
      <Router fallback=|| {
        let mut outside_errors = Errors::default();
        outside_errors.insert_with_default_key(AppError::NotFound);
        view! { <ErrorTemplate outside_errors /> }.into_view()
      }>
        <main>
          <Routes>
            <Route path="" view=Page>
              // Only accessible when logged in
              <Route path="" view=AuthenticatedRoutes>
                <Route path="/AdminHomePage" view=AdminHomePage />
                <Route path="/classes" view=ClassesPage />
                <Route path="/classes/:class_id" view=ClassPage>
                  <Route path="" view=|| {} />
                  <Route path="/details" view=ClassDetails />
                  <Route path="/:post_id" view=FocusedPost />
                  <Route path="/:post_id/edit" view=EditPost />
                  <Route path="/announcement/:announcement_id" view=AnnouncementDetails />
                </Route>
                <Route path="/class/:class_id/poll" view=LivePoll />
                <Route path="/settings" view=UserSettings />
                <Route path="/profile" view=UserProfile />
              </Route>
              // Accessible when logged out
              <Route path="" view=UnauthenticatedRoutes>
                <Route path="" view=Home />
                <Route path="/about-us" view=AboutUs />
                <Route path="/tutorial" view=Tutorial />
                <Route path="/login" view=LoginPage />
                <Route path="/register" view=RegisterPage />
              </Route>
            </Route>
          </Routes>
        </main>
      </Router>
    }
}

/// Its pretty safe to use expect_context because we provide this context in the root of the app
pub fn expect_auth_context() -> AuthContext {
    expect_context::<AuthContext>()
}

/// Returns a user if the user is logged in otherwise redirects to the login page
#[macro_export]
macro_rules! expect_logged_in_user {
    () => {{
        use leptos::SignalGetUntracked;
        let auth_context = $crate::app::expect_auth_context();
        if auth_context.get_untracked().is_unauthenticated() {
            let navigate = leptos_router::use_navigate();
            navigate("/login", Default::default());
            return view! { Redirecting to login... }.into_view();
        }

        leptos::create_slice(
            auth_context,
            |auth| {
                if let $crate::data::global_state::Authentication::Authenticated(user) = auth {
                    user.clone()
                } else {
                    panic!("User is not authenticated. expect_logged_in_user! isn't working.")
                }
            },
            |auth_context, new_user: $crate::data::global_state::UserBuilder| {
                if let $crate::data::global_state::Authentication::Authenticated(ref mut user) =
                    auth_context
                {
                    let $crate::data::global_state::UserBuilder {
                        first_name,
                        last_name,
                        user_name,
                        id,
                        role,
                    } = new_user;

                    if let Some(first_name) = first_name {
                        user.first_name = first_name;
                    }
                    if let Some(last_name) = last_name {
                        user.last_name = last_name;
                    }
                    if let Some(user_name) = user_name {
                        user.user_name = user_name;
                    }
                    if let Some(id) = id {
                        user.id = id;
                    }
                    if let Some(role) = role {
                        user.role = role;
                    }
                }
            },
        )
    }};
}

/// Checks for authentication embedded in the window context by the server
fn provide_authentication_from_window_context() {
    #[cfg(not(feature = "ssr"))]
    {
        use crate::data::global_state::User;

        let ssr_embedded_user = web_sys::window()
            .expect("should have a window in this context")
            .get("__EMBEDDED_ENCAMPUS_AUTHENTICATION__")
            .and_then(|x| x.as_string())
            .and_then(|serialized_user| serde_json::from_str::<User>(&serialized_user).ok());
        let auth = match ssr_embedded_user {
            Some(user) => Authentication::Authenticated(user),
            None => Authentication::Unauthenticated,
        };
        provide_context::<AuthContext>(RwSignal::new(auth));
    }
}

// [Route wrappers] ===============================================

/// Prevent errors due to loading pages that require authentication while logged
/// out
#[component]
pub fn AuthenticatedRoutes() -> impl IntoView {
    let auth_context = expect_auth_context();

    // Start the timer
    start_timer();

    move || match auth_context.get() {
        Authentication::Authenticated(_) => view! { <Outlet /> }.into_view(),
        Authentication::Unauthenticated => {
            create_render_effect(|_| {
                let navigate = use_navigate();
                navigate("/login", Default::default());
            });
            view! { Redirecting to login... }.into_view()
        }
    }
}

/// General route wrapping
#[component]
fn UnauthenticatedRoutes() -> impl IntoView {
    view! {
      <div class="w-auto h-auto bg-cover bg-[url('/images/pattern_bg.jpg')]">
        <Outlet />
      </div>
    }
}

// [Application-wide helpers] ===============================================

/// Starts a persistent clock. Configures notifications once, then calls create_push_notification on a loop
fn start_timer() {
    #[cfg(target_arch = "wasm32")]
    {
        use gloo_timers::callback::Interval;
        use crate::pages::global_components::push_notifications::{configure_notifications, send_newest_announcement_notification};
        use web_sys::window;

        // Timer signals
        let (tick_counter, set_tick_counter) = create_signal(0);

        // Timer interval
        let interval = Interval::new(3000, move || {
            set_tick_counter.update(|v| *v += 1);
        });

        // Configure push notifications once
        spawn_local(async {
            if let Some(window) = window() {
                if let Err(err) = configure_notifications(&window).await {
                    logging::log!("Notification handling error: {:?}", err);
                }
            }
        });

        // Listen for timer changes
        create_effect(move |_| {
            let _tick_value = tick_counter.get();

            // Call the send_newest_announcement_notification function
            spawn_local(async move {
                if let Err(err) = send_newest_announcement_notification().await {
                    logging::log!("Failed to send announcement notification: {:?}", err);
                }
            });
        });

        // Interval will be dropped at the end of this scope, cancelling itself
        leptos::on_cleanup(move || drop(interval));
    }
}
