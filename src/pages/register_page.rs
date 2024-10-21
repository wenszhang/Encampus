use crate::data::database::user_functions::add_user;
use crate::data::database::user_functions::User;
use crate::data::global_state::GlobalState;
use crate::pages::global_components::notification::{
    NotificationComponent, NotificationDetails, NotificationType,
};
/**
 * Component for the login page where users can login to their account
 */
use leptos::{ev::SubmitEvent, *};

#[component]
pub fn RegisterPage() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    let (first_name, set_first_name) = create_signal("".to_string());
    let (last_name, set_last_name) = create_signal("".to_string());
    let (user_id, set_user_id) = create_signal(0);
    let (password, set_password) = create_signal("".to_string());
    let (confirm_password, set_confirm_password) = create_signal("".to_string());
    let (login_error, set_login_error) = create_signal(None::<NotificationDetails>);

    let on_input = |setter: WriteSignal<String>| {
        move |ev| {
            setter(event_target_value(&ev));
        }
    };

    let new_user = User {
        username: username.get(),
        firstname: first_name.get(),
        lastname: last_name.get(),
        role: "Student".to_string(),
        id: 0,
    };

    let new_user_action = create_action(move |_| async move {
        if password.get() != confirm_password.get() {
            set_login_error(Some(NotificationDetails {
                message: "Passwords do not match.".to_string(),
                notification_type: NotificationType::Error,
            }));
        } else {
            match add_user(
                User {
                    username: username.get(),
                    firstname: first_name.get(),
                    lastname: last_name.get(),
                    role: "Student".to_string(),
                    id: 0,
                },
                password.get(),
            )
            .await
            {
                Ok(user) => {
                    set_user_id(user.id);
                }
                Err(_) => {
                    set_login_error(Some(NotificationDetails {
                        message: "Failed adding user, username already exists.".to_string(),
                        notification_type: NotificationType::Error,
                    }));
                }
            }
        }
    });

    create_effect(move |_| {
        let global_state = expect_context::<GlobalState>();
        if let Some(_id) = new_user_action.value()() {
            if user_id.get() > 0 {
                global_state.authenticated.set(true);
                global_state.user_name.set(Some(username.get()));
                global_state.id.set(Some(user_id.get()));
                global_state.first_name.set(Some(first_name.get()));
                global_state.last_name.set(Some(last_name.get()));
                global_state.role.set(Some("Student".to_string()));

                let navigate = leptos_router::use_navigate();
                match global_state.role.get().unwrap_or_default().as_str() {
                    "Student" => navigate("/classes", Default::default()),
                    // Change to instructor page when implemented
                    "Instructor" => navigate("/classes", Default::default()),
                    // Change to admin page when implemented
                    "Admin" => navigate("/classes", Default::default()),
                    _ => navigate("/login", Default::default()),
                }
            }
        }
    });

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        new_user_action.dispatch(new_user.clone());
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
              <label for="password" class="flex row-auto mb-2 font-bold text-gray-700">
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
              <label for="confirm_password" class="flex row-auto mb-2 font-bold text-gray-700">
                Confirm Password:
              </label>
              <input
                type="password"
                id="confirm_password"
                placeholder="Confirm your Password"
                required
                class="py-2 px-3 w-full rounded-md border border-gray-300 focus:border-blue-500 focus:outline-none"
                on:input=on_input(set_confirm_password)
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
