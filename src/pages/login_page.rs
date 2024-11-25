/**
 * Component for the login page where users can login to their account
 */
use crate::app::expect_auth_context;
use crate::data::database::user_functions::login;
use crate::data::global_state::{Authentication, User};
use crate::on_input;
use crate::pages::global_components::notification::{
    NotificationComponent, NotificationDetails, NotificationType,
};
use leptos::{ev::SubmitEvent, *};

struct LoginText {
    username: String,
    password: String,
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (login_error, set_login_error) = create_signal(None::<NotificationDetails>);
    let auth_context = expect_auth_context();

    let login_action = create_action(|LoginText { username, password }| {
        let username = username.to_owned();
        let password = password.to_owned();
        async move {
            login(username, password)
                .await
                .map(|user_option| {
                    user_option.map(|dbUser| User {
                        id: dbUser.id,
                        first_name: dbUser.firstname,
                        last_name: dbUser.lastname,
                        user_name: dbUser.username,
                        role: dbUser.role,
                    })
                })
                .map_err(|_server_fn_err| "An error occurred. Please try again")
        }
    });

    create_effect(move |_| {
        match login_action.value()() {
            // action hasn't finished yet
            None => {}
            // an error occurred during login request
            Some(Err(message)) => {
                set_login_error.set(Some(NotificationDetails {
                    message: message.to_string(),
                    notification_type: NotificationType::Error,
                }));
            }
            // login returned no user
            Some(Ok(None)) => {
                set_login_error.set(Some(NotificationDetails {
                    message: "Failed Signing In: No user found. Please try again".to_string(),
                    notification_type: NotificationType::Error,
                }));
            }
            // login returned a user :)
            Some(Ok(Some(user))) => {
                let role = user.role.clone();
                // set global authentication context
                auth_context.set(Authentication::Authenticated(user));

                // Navigate based on the user's role
                let navigate = leptos_router::use_navigate();
                match role.as_str() {
                    "Student" => navigate("/classes", Default::default()),
                    // Change to instructor page when implemented
                    "Instructor" => navigate("/classes", Default::default()),
                    // Change to admin page when implemented
                    "Admin" => navigate("/AdminHomePage", Default::default()),
                    _ => set_login_error.set(Some(NotificationDetails {
                        message: format!("A unexpected user role was encountered on this user. Failing to redirect. Role: {}", role),
                        notification_type: NotificationType::Error,
                    }))
                }
            }
        }
    });

    // Form submission handler
    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        login_action.dispatch(LoginText {
            username: username(),
            password: password(),
        });
    };

    let notification_view = move || {
        login_error().map(|details| {
            view! {
              <NotificationComponent
                notification_details=details.clone()
                on_close=move || set_login_error(None)
              />
            }
        })
    };

    view! {
      <form on:submit=on_submit>
        <div class="flex flex-col justify-center items-center h-screen">
          <div class="p-20 w-96 bg-white rounded-lg shadow-md">
            <div class="flex justify-center items-center">
              <img src=format!("/{}", "images/logo.png") alt="Logo" class="h-16" />
            </div>
            <h1 class="mb-4 text-2xl font-semibold text-center">ENCAMPUS</h1>
            {notification_view}
            <div class="mb-4">
              <label for="username" class="block mb-2 font-bold text-gray-700">
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
              <label for="password" class="block mt-2 font-bold text-gray-700">
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
            </div>
            <button
              type="submit"
              class="py-2 px-4 w-full text-white rounded-md focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue"
            >
              Submit
            </button>
            <div class="mt-4 text-sm text-center text-gray-600">Please enter your username.</div>
            <div class="mt-1 text-sm text-center text-gray-600">
              Click here to <a href="/register" class="text-blue-500">register</a>.
            </div>
          </div>
        </div>
      </form>
    }
}
