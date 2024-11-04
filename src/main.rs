/*
 * This file is the entry point for the server-side application.
 */
cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {

        use axum::{
            extract::ws::{Message, WebSocket, WebSocketUpgrade},
            response::{IntoResponse, Response},
            routing::{get, post},
            Router,
        };
        use tokio::sync::broadcast;
        use http::StatusCode;

        use axum::extract::{Request, State, FromRef};
        use encampus::app::*;
        use encampus::fileserv::file_and_error_handler;
        use leptos::*;
        use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context, render_route_with_context};
        use leptos_router::RouteListing;
        use tower_http::trace::TraceLayer;
        use sqlx::postgres::{PgPoolOptions, PgPool};

        use encampus::data::database::db_notifications::listen_for_db_notifications;
        use encampus::data::database::announcement_functions::post_announcement;
        use encampus::data::database::announcement_functions::AddAnnouncementInfo;

        /// A collection of application state information used to supply any outside information needed by Axum handlers.
        #[derive(FromRef, Debug, Clone)]
        struct AppState {
            leptos_options: LeptosOptions,
            app_routes: Vec<RouteListing>,
            pool: PgPool,
            // used to send messages to connected clients.
            tx: broadcast::Sender<String>,
        }

        /// Handler for the WebSocket connection.
        async fn websocket_handler(
            ws: WebSocketUpgrade,
            State(state): State<AppState>,
        ) -> impl IntoResponse {
            ws.on_upgrade(move |socket| handle_socket(socket, state))
        }

        /// Handle individual WebSocket connections
        async fn handle_socket(mut socket: WebSocket, state: AppState) {

            let mut rx = state.tx.subscribe();

            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        // If a new announcement or post is published, broadcast the update to the WebSocket clients
                        Ok(message) = rx.recv() => {
                            if socket.send(Message::Text(message)).await.is_err() {
                                break;
                            }
                        }
                    }
                }
            });
        }

        async fn post_announcement_handler(
            State(state): State<AppState>,
            new_announcement: AddAnnouncementInfo,  // Directly handle the struct
            user_id: i32,  // Include user_id as needed
        ) -> impl IntoResponse {
            // Save the announcement to the database
            match post_announcement(new_announcement, user_id).await {
                Ok(announcement) => {
                    // Broadcast the new announcement title via WebSocket
                    let _ = state.tx.send(format!("New announcement: {}", announcement.title));

                    // Respond to the HTTP request
                    (StatusCode::OK, "Announcement posted")
                }
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to post announcement"),
            }
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

            let (tx, _rx) = broadcast::channel::<String>(100);

            // Setting get_configuration(None) means we'll be using cargo-leptos's env values
            let conf = get_configuration(None).await.unwrap();
            let leptos_options = conf.leptos_options;
            let addr = leptos_options.site_addr;
            let state = AppState {
                leptos_options,
                app_routes: generate_route_list(App),
                pool: pool.clone(),
                tx: tx.clone(),
            };

            // Spawn the notification listener in a background task for announcements, posts, etc.
            let pool_clone = pool.clone();
            let tx_clone = tx.clone();
            tokio::spawn(async move {
                listen_for_db_notifications(pool_clone, tx_clone).await;
            });

            // build our application with a route
            let app = Router::new()
                .route("/ws", get(websocket_handler))  // WebSocket endpoint
                .route("/post_announcement", post(post_announcement_handler))  // HTTP post for announcements
                .route("/new_post", post(post_announcement_handler))  // HTTP post for new posts
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
