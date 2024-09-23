use crate::data::database::user_functions::{add_user, User};
use crate::data::global_state::GlobalState;
use crate::pages::global_components::notification::{
    NotificationComponent, NotificationDetails, NotificationType,
};
/**
 * Component for the registration page where users can create a new account
 */
use leptos::{ev::SubmitEvent, *};

#[component]
pub fn RegisterPage() -> impl IntoView {
    // Signals for form inputs
    let (username, set_username) = create_signal("".to_string());
    let (first_name, set_first_name) = create_signal("".to_string());
    let (last_name, set_last_name) = create_signal("".to_string());
    let (user_id, set_user_id) = create_signal(0);
    let (login_error, set_login_error) = create_signal(None::<NotificationDetails>);

    // Input event handler for controlled components
    let on_input = |setter: WriteSignal<String>| {
        move |ev| {
            setter(event_target_value(&ev));
        }
    };

    // Action to add a new user
    let new_user_action = create_action(move |_| async move {
        let user = User {
            username: username.get(),
            firstname: first_name.get(),
            lastname: last_name.get(),
            role: "student".to_string(),
            id: 0,
        };
        match add_user(user).await {
            Ok(id) => {
                set_user_id(id);
            }
            Err(_) => {
                set_login_error(Some(NotificationDetails {
                    message: "Failed adding user, username already exists.".to_string(),
                    notification_type: NotificationType::Error,
                }));
            }
        }
    });

    // Effect to update global state and navigate after successful registration
    create_effect(move |_| {
        let global_state = expect_context::<GlobalState>();
        if new_user_action.value().with(|v| v.is_some()) && user_id.get() > 0 {
          // Update the user_state with new values
          global_state.user_state.update(|state| {
              state.authenticated = true;
              state.user_name = Some(username.get());
              state.id = Some(user_id.get());
              state.first_name = Some(first_name.get());
              state.last_name = Some(last_name.get());
              state.role = Some("student".to_string());
          });

          // Save user info to local storage
          global_state.save_to_local_storage();

          // Navigate based on the user's role
          let navigate = leptos_router::use_navigate();
          let role = global_state.user_state.get().role.clone().unwrap_or_default();

          match role.as_str() {
              "student" => navigate("/classes", Default::default()),
              // Change to instructor page when implemented
              "instructor" => navigate("/classes", Default::default()),
              // Change to admin page when implemented
              "admin" => navigate("/classes", Default::default()),
              _ => navigate("/login", Default::default()),
          }
        }
    });

    // Form submission handler
    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        new_user_action.dispatch(());
    };

    // View for displaying notifications
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
                on:input=on_input(set_username)
                prop:value=username
              />
            </div>
            <div class="mb-4">
              <label for="first_name" class="flex row-auto mb-2 font-bold text-gray-700">
                First Name:
              </label>
              <input
                type="text"
                id="first_name"
                placeholder="First Name"
                required
                class="py-2 px-3 w-full rounded-md border border-gray-300 focus:border-blue-500 focus:outline-none"
                on:input=on_input(set_first_name)
                prop:value=first_name
              />
            </div>
            <div class="mb-4">
              <label for="last_name" class="flex row-auto mb-2 font-bold text-gray-700">
                Last Name:
              </label>
              <input
                type="text"
                id="last_name"
                placeholder="Last Name"
                required
                class="py-2 px-3 w-full rounded-md border border-gray-300 focus:border-blue-500 focus:outline-none"
                on:input=on_input(set_last_name)
                prop:value=last_name
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