/*
 * This file is the entry point for the server-side application.
 */
cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use axum::response::{IntoResponse, Response};
        use axum::extract::{Request, State, FromRef};
        use axum::routing::get;
        use axum::Router;
        use encampus::app::*;
        use encampus::fileserv::file_and_error_handler;
        use leptos::*;
        use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context, render_route_with_context};
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
        async fn server_fn_handler(State(app_state): State<AppState>, request: Request) -> impl IntoResponse {
            handle_server_fns_with_context(move || provide_context(app_state.pool.clone()), request).await
        }

        /// A Axum handler to wrap all leptos routes and provide them with a database connection using provide_context.
        async fn leptos_routes_handler(State(app_state): State<AppState>, req: Request) -> Response {
            let handler = render_route_with_context(
                app_state.leptos_options.clone(),
                generate_route_list(App),
                move || provide_context::<PgPool>(app_state.pool.clone()),
                App
            );
            handler(req).await
        }

        #[tokio::main]
        async fn main() {
            // Uncomment to turn on detailed logging for all crates that use the tracing crate.
            // tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect("postgres://dev:encampus@143.198.110.69/encampusdb")
                .await
                .expect("Database at 143.198.110.69 should be live and accessible");

            // Setting get_configuration(None) means we'll be using cargo-leptos's env values
            // For deployment these variables are:
            // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
            // Alternately a file can be specified such as Some("Cargo.toml")
            // The file would need to be included with the executable when moved to deployment
            let conf = get_configuration(None).await.unwrap();
            let leptos_options = conf.leptos_options;
            let addr = leptos_options.site_addr;
            let state = AppState { leptos_options, app_routes: generate_route_list(App), pool: pool.clone() };

            // build our application with a route
            let app = Router::new()
                .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
                .leptos_routes_with_handler(state.app_routes.clone(), leptos_routes_handler)
                .fallback(file_and_error_handler)
                .with_state(state)
                .layer(TraceLayer::new_for_http());

            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
            logging::log!("listening on http://{}", &addr);
            axum::serve(listener, app.into_make_service())
                .await
                .unwrap();
        }
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
