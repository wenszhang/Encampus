pub mod app;
pub mod data;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod pages;
pub mod resources;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use data::global_state::{Authentication, User};
        use tower_sessions::Session;

        #[derive(Clone)]
        pub struct AuthenticationSession(Session);

        impl AuthenticationSession {
            const USER_KEY: &'static str = "user";
            pub fn new(session_store: Session) -> Self {
                Self(session_store)
            }
            pub async fn login(&self, user: User) {
                self.0
                    .insert(AuthenticationSession::USER_KEY, user)
                    .await
                    .expect("Memory Store should always be available for insertion");
            }
            pub async fn get_authentication(&self) -> Authentication {
                match self
                    .0
                    .get::<User>(AuthenticationSession::USER_KEY)
                    .await
                    .expect("Memory Store should always be available and have a valid user")
                {
                    Some(user) => Authentication::Authenticated(user),
                    None => Authentication::Unauthenticated,
                }
            }
            pub async fn logout(&self) {
                self.0
                    .delete()
                    .await
                    .expect("Memory Store should always be available for deletion");
            }
        }
    }
}
