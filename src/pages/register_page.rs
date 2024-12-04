/**
 * Component for the login page where users can login to their account
 */
use crate::app::expect_auth_context;
use crate::data::database::user_functions::add_user;
use crate::data::global_state::Authentication;
use crate::data::global_state::User;
use crate::pages::global_components::notification::{
    NotificationComponent, NotificationDetails, NotificationType,
};
use leptos::{ev::SubmitEvent, *};
use serde::Deserialize;
use serde::Serialize;

#[macro_export]
macro_rules! on_input {
    ($setter:ident) => {
        move |ev: web_sys::Event| {
            $setter(leptos::event_target_value(&ev));
        }
    };
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewUser {
    pub user: User,
    pub password: String,
}

#[component]
pub fn RegisterPage() -> impl IntoView {
    let auth_context = expect_auth_context();
    let (username, set_username) = create_signal("".to_string());
    let (first_name, set_first_name) = create_signal("".to_string());
    let (last_name, set_last_name) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (confirm_password, set_confirm_password) = create_signal("".to_string());
    let (login_error, set_login_error) = create_signal(None::<NotificationDetails>);

    let new_user_action = create_action(move |new_user: &NewUser| {
        let new_user = new_user.to_owned();
        async move {
            add_user(new_user, true)
                .await
                .map(|dbUser| User {
                    id: dbUser.id,
                    user_name: dbUser.username,
                    first_name: dbUser.firstname,
                    last_name: dbUser.lastname,
                    role: dbUser.role,
                })
                .map_err(|_server_fn_err| "An error occurred. Please try again")
        }
    });

    create_effect(move |_| {
        match new_user_action.value()() {
            None => {}
            Some(Err(message)) => {
                set_login_error.set(Some(NotificationDetails {
                    message: message.to_string(),
                    notification_type: NotificationType::Error,
                }));
            }
            Some(Ok(user)) => {
                auth_context.set(Authentication::Authenticated(user.clone()));

                let navigate = leptos_router::use_navigate();
                match user.role.as_str() {
                    "Student" => navigate("/classes", Default::default()),
                    // Change to instructor page when implemented
                    "Instructor" => navigate("/classes", Default::default()),
                    // Change to admin page when implemented
                    "Admin" => navigate("/classes", Default::default()),
                    _ => set_login_error.set(Some(NotificationDetails {
                      message: format!("A unexpected user role was encountered on this user. Failing to redirect. Role: {}", user.role),
                      notification_type: NotificationType::Error,
                  }))
                }
            }
        }
    });

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        if password() != confirm_password() {
            set_login_error(Some(NotificationDetails {
                message: "Passwords do not match.".to_string(),
                notification_type: NotificationType::Error,
            }));
            return;
        }
        new_user_action.dispatch(NewUser {
            user: User {
                first_name: first_name(),
                last_name: last_name(),
                user_name: username(),
                role: "Student".to_string(),
                id: 0,
            },
            password: password(),
        });
    };

    let notification_view = move || {
        login_error.get().map(|details| {
            view! {
            <div class="w-full">  // Add a full-width container
              <NotificationComponent
                notification_details=details.clone()
                on_close=move || set_login_error(None)
              />
            </div>
                }
        })
    };

    view! {
      <form on:submit=on_submit class="opacity-95">
        <div class="flex flex-col justify-center items-center h-screen">
          <div class="p-20 w-96 bg-white rounded-lg shadow-md">
            <div class="flex justify-center items-center">
              <img src=format!("/{}", "images/logo.png") alt="Logo" class="h-16" />
            </div>
            <h1 class="mb-2 text-2xl font-semibold text-center">Create Account</h1>
            <div class="mt-4 mb-4 text-sm text-center text-gray-600">
              Please fill out the following information to create an account.
            </div>
            {notification_view}
            <div class="mb-4">
              <label for="username" class="flex row-auto mb-2 font-bold text-gray-700">
                Username:
              </label>
              <input
                type="text"
                id="username"
                placeholder="Enter your Username"
                required
                class="py-2 px-3 w-full rounded-md border border-gray-300 focus:border-blue-500 focus:outline-none"
                on:input=on_input!(set_username)
                prop:value=username
              />
              <label for="first_name" class="flex row-auto mb-2 font-bold text-gray-700">
                First Name:
              </label>
              <input
                type="text"
                id="first_name"
                placeholder="First Name"
                required
                class="py-2 px-3 w-full rounded-md border border-gray-300 focus:border-blue-500 focus:outline-none"
                on:input=on_input!(set_first_name)
                prop:value=first_name
              />
              <label for="last_name" class="flex row-auto mb-2 font-bold text-gray-700">
                Last Name:
              </label>
              <input
                type="text"
                id="last_name"
                placeholder="Last Name"
                required
                class="py-2 px-3 w-full rounded-md border border-gray-300 focus:border-blue-500 focus:outline-none"
                on:input=on_input!(set_last_name)
                prop:value=last_name
              />
              <label for="password" class="flex row-auto mb-2 font-bold text-gray-700">
                Password:
              </label>
              <input
                type="password"
                id="password"
                placeholder="Enter your Password"
                required
                class="py-2 px-3 w-full rounded-md border border-gray-300 focus:border-blue-500 focus:outline-none"
                on:input=on_input!(set_password)
                prop:value=password
              />
              <label for="confirm_password" class="flex row-auto mb-2 font-bold text-gray-700">
                Confirm Password:
              </label>
              <input
                type="password"
                id="confirm_password"
                placeholder="Confirm your Password"
                required
                class="py-2 px-3 w-full rounded-md border border-gray-300 focus:border-blue-500 focus:outline-none"
                on:input=on_input!(set_confirm_password)
                prop:value=confirm_password
              />
            </div>
            <button
              type="submit"
              class="py-2 px-4 w-full text-white bg-blue-500 rounded-md hover:bg-blue-600 focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:outline-none"
            >
              Register
            </button>
            <div class="mt-4 text-sm text-center text-gray-600">
              Already have an account? <a href="/login" class="text-blue-500">
                Login
              </a>
            </div>
          </div>
        </div>
      </form>
    }
}
