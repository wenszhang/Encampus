pub mod ai_functions;
pub mod announcement_functions;
pub mod class_functions;
pub mod live_poll_functions;
pub mod post_functions;
pub mod reply_functions;
pub mod user_functions;

#[macro_export]
macro_rules! expect_authenticated_user_or_redirect_to_login {
    () => {{
        use leptos::expect_context;
        use leptos::server_fn::error::NoCustomError;
        use leptos::ServerFnError;
        use leptos_axum::navigate;
        use $crate::AuthenticationSession;

        const auth = expect_context::<AuthenticationSession>()
            .get_authentication().await;
        match auth {
            Authentication::Unauthenticated => {

                navigate("/login");
                return ServerFnError::<NoCustomError>::ServerError("User is not authenticated".to_string());
            }
            Authentication::Authenticated(user) => user,
        }
    }};
}
