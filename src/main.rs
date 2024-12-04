/*
 * This file is the entry point for the server-side application.
 */
cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use encampus::data::global_state::{Authentication, User};
        use leptos_axum::generate_route_list_with_exclusions_and_ssg_and_context;
        use tower_sessions::{cookie::{Key, time::Duration}, Session};
        use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
        use axum::response::{IntoResponse, Response};
        use axum::extract::{Request, State, FromRef};
        use axum::routing::get;
        use axum::Router;
        use encampus::app::*;
        use encampus::AuthenticationSession;
        use encampus::data::global_state::AuthContext;
        use encampus::fileserv::file_and_error_handler;
        use leptos::*;
        use leptos_axum::{LeptosRoutes, handle_server_fns_with_context, render_route_with_context};
        use leptos_router::RouteListing;
        use tower_http::trace::TraceLayer;
        use sqlx::postgres::{PgPoolOptions, PgPool};

        /// A collection of application state information used to supply any outside information needed by Axum handlers.
        // Derive FromRef to allow multiple items in state, using Axum’s
        // SubStates pattern.
        #[derive(FromRef, Debug, Clone)]
        struct AppState {
            leptos_options: LeptosOptions,
            app_routes: Vec<RouteListing>,
            pool: PgPool,
        }

        /// A Axum handler specifically to wrap leptos server functions and provide them with a database connection using provide_context.
        async fn server_fn_handler(State(app_state): State<AppState>, session_store: Session, request: Request) -> impl IntoResponse {
            let auth_session = AuthenticationSession::new(session_store.clone());
            let auth = auth_session.get_authentication().await;

            handle_server_fns_with_context(
                move || {
                    provide_context::<PgPool>(app_state.pool.clone());
                    provide_context::<AuthContext>(RwSignal::new(auth.clone()));
                    provide_context(AuthenticationSession::new(session_store.clone()));
                },
                request
            ).await
        }

        /// A Axum handler to wrap all leptos routes and provide them with a database connection using provide_context.
        async fn leptos_routes_handler(State(app_state): State<AppState>, session_store: Session, req: Request) -> Response {
            let auth_session = AuthenticationSession::new(session_store.clone());
            let auth = auth_session.get_authentication().await;

            let handler = render_route_with_context(
                app_state.leptos_options.clone(),
                app_state.app_routes.clone(),
                move || {
                    provide_context::<PgPool>(app_state.pool.clone());
                    provide_context::<AuthContext>(RwSignal::new(auth.clone()));
                    provide_context(AuthenticationSession::new(session_store.clone()));
                },
                App
            );
            handler(req).await
        }

        #[tokio::main]
        async fn main() {
            // Uncomment to turn on detailed logging for all crates that use the tracing crate.
            // tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

            // Connect to the database
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect("postgres://encampus:encampus@localhost/encampusdb")
                .await
                .expect("Database at localhost:5432 should be live and accessible");

            // Set up session management for auth
            let session_store = MemoryStore::default();
            let session_layer = SessionManagerLayer::new(session_store).with_private(Key::generate())
                .with_expiry(Expiry::OnInactivity(Duration::hours(1))).with_always_save(true);

            // Setting get_configuration(None) means we'll be using cargo-leptos's env values
            // For deployment these variables are:
            // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
            // Alternately a file can be specified such as Some("Cargo.toml")
            // The file would need to be included with the executable when moved to deployment
            let conf = get_configuration(None).await.unwrap();
            let leptos_options = conf.leptos_options;
            let addr = leptos_options.site_addr;
            let state = AppState { leptos_options, app_routes: get_app_routes(), pool: pool.clone() };

            // build our application with a route
            let app = Router::new()
                .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
                .leptos_routes_with_handler(state.app_routes.clone(), leptos_routes_handler)
                .fallback(file_and_error_handler)
                .with_state(state)
                .layer(TraceLayer::new_for_http())
                .layer(session_layer);

            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
            logging::log!("listening on http://{}", &addr);
            axum::serve(listener, app.into_make_service())
                .await
                .unwrap();
        }

        fn get_app_routes() -> Vec<RouteListing> {
            generate_route_list_with_exclusions_and_ssg_and_context(App, None, || {
                provide_context::<AuthContext>(RwSignal::new(Authentication::Authenticated(User {
                    id: 696969,
                    first_name: "dummy".to_string(),
                    last_name: "dummy".to_string(),
                    user_name: "dummy".to_string(),
                    role: "admin".to_string(),
                })));
            }).0
        }
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
