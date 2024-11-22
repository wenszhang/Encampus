pub mod ai_functions;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

pub mod announcement_functions;
pub mod class_functions;
pub mod post_functions;
pub mod reply_functions;
pub mod user_functions;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum EncampusServerError {
    AuthenticationError,
    UnrecoverableServerError,
}

impl Display for EncampusServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncampusServerError::AuthenticationError => {
                write!(f, "AuthenticationError")
            }
            EncampusServerError::UnrecoverableServerError => {
                write!(f, "UnrecoverableServerError")
            }
        }
    }
}

impl FromStr for EncampusServerError {
    type Err = enum {ParseError};

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AuthenticationError" => Ok(EncampusServerError::AuthenticationError),
            "UnrecoverableServerError" => Ok(EncampusServerError::UnrecoverableServerError),
            _ => Err(Slef::Err),
        }
    }
    
}

#[cfg(feature = "ssr")]
#[macro_export]
macro_rules! expect_authenticated_user_or_redirect_to_login {
    () => {
        match async {
            use leptos::expect_context;
            use $crate::Authentication;
            use $crate::AuthenticationSession;

            let auth = expect_context::<AuthenticationSession>()
                .get_authentication()
                .await;
            match auth {
                Authentication::Unauthenticated => None,
                Authentication::Authenticated(user) => {
                    console_log(format!("{:?}", user).as_str());
                    Some(user)
                }
            }
        }
        .await
        {
            Some(user) => user,
            None => {
                use leptos::ServerFnError;
                console_log("User is not authenticated");
                leptos_axum::redirect("/login");
                return Err(
                    ServerFnError::<crate::data::database::EncampusServerError>::WrappedServerError(
                        crate::data::database::EncampusServerError::AuthenticationError,
                    ),
                );
            }
        }
    };
}
