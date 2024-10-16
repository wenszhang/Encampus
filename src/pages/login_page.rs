/**
 * Component for the login page where users can login to their account
 */
use leptos::{ev::SubmitEvent, *};

use crate::data::{database::user_functions::login, global_state::GlobalState};
use crate::pages::global_components::notification::{
    NotificationComponent, NotificationDetails, NotificationType,
};

#[component]
pub fn LoginPage() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (login_error, set_login_error) = create_signal(None::<NotificationDetails>);

    // Input event handler for controlled components
    let on_input = |setter: WriteSignal<String>| {
        move |ev| {
            setter(event_target_value(&ev));
        }
    };

    let login_action = create_action(move |_| {
        let username = username.get().to_owned();
        let password = password.get().to_owned();
        async move {
            match login(username.clone(), password.clone()).await{
                Ok(user) => (Some(user.username), Some(user.id), Some(user.firstname), Some(user.lastname), Some(user.role), Some(None)),
                Err(ServerFnError::ServerError(err_msg)) =>  (None, None, None, None, None, Some(Some(err_msg))),
                Err(_) => (None, None, None, None, None, Some(Some("Unknown error".to_string()))),
            }
      }
    });

    create_effect(move |_| {
        let global_state = expect_context::<GlobalState>();
        if let Some(userInfo) = login_action.value()() {
            if let Some(Some(error_msg)) = userInfo.5 {
              set_login_error.set(Some(NotificationDetails {
                message: format!("Failed Signing In: {}", error_msg),
                notification_type: NotificationType::Error,
            }));
            } else {
                global_state.authenticated.set(true);
                global_state.user_name.set(Some(userInfo.0.unwrap_or_default()));
                global_state.id.set(Some(userInfo.1.unwrap_or_default()));
                global_state.first_name.set(Some(userInfo.2.unwrap_or_default()));
                global_state.last_name.set(Some(userInfo.3.unwrap_or_default()));
                global_state.role.set(Some(userInfo.4.unwrap_or_default()));

                // Save user info to local storage
                global_state.save_to_local_storage();

                // Navigate based on the user's role
                let navigate = leptos_router::use_navigate();
                match global_state.role.get().unwrap_or_default().as_str() {
                    "student" => navigate("/classes", Default::default()),
                    // Change to instructor page when implemented
                    "instructor" => navigate("/classes", Default::default()),
                    // Change to admin page when implemented
                    "admin" => navigate("/AdminHomePage", Default::default()),
                    _ => navigate("/login", Default::default()),
                }
            }
        }
    });

    // Form submission handler
    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        login_action.dispatch(username());
    };

    let notification_view = move || {
        login_error.get().map(|details| {
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
            <h1 class="mb-4 text-2xl font-semibold text-center">Login</h1>
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
                on:input=on_input(set_username)
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
                on:input=on_input(set_password)
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
              Click here to <a href="/register" class="text-blue-500">
                register
              </a>.
            </div>
          </div>
        </div>
      </form>
    }
}
