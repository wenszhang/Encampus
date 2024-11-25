use crate::data::database::user_functions::{update_user_password, update_user_without_password};
use crate::data::global_state::User;
use crate::expect_logged_in_user;
use crate::pages::global_components::header::Header;
use crate::pages::global_components::notification::{
    NotificationComponent, NotificationDetails, NotificationType,
};
use crate::pages::global_components::sidebar::Sidebar;
use leptos::ev::SubmitEvent;
use leptos::{
    component, create_action, create_effect, create_signal, view, IntoView, Signal,
    SignalGetUntracked, Suspense,
};
use leptos_router::use_navigate;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

/// Renders the user settings page
#[component]
pub fn UserSettings() -> impl IntoView {
    let (user, _) = expect_logged_in_user!();
    let (user_name, set_user_name) = create_signal(user.get_untracked().user_name);
    let (password, set_password) = create_signal(String::new());
    let (confirm_password, set_confirm_password) = create_signal(String::new());
    let (update_error, set_update_error) = create_signal(None::<NotificationDetails>);

    let update_user_action = create_action(
        move |(updated_user, new_password): &(User, Option<String>)| {
            let updated_user = updated_user.clone();
            let new_password = new_password.clone();
            async move {
                if let Some(password) = new_password {
                    update_user_password(updated_user.id, password)
                        .await
                        .map_err(|_| {
                            "An error occurred while updating the password. Please try again"
                        })
                } else {
                    update_user_without_password(updated_user)
                        .await
                        .map(|_| ())
                        .map_err(|_server_fn_err| "An error occurred. Please try again")
                }
            }
        },
    );

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        let mut user = user.get_untracked();
        user.user_name = user_name.get_untracked().clone();
        if password.get_untracked() == confirm_password.get_untracked() {
            let new_password = if password.get_untracked().is_empty() {
                None
            } else {
                Some(password.get_untracked().clone())
            };
            update_user_action.dispatch((user, new_password));
        } else {
            set_update_error(Some(NotificationDetails {
                message: "Passwords do not match".to_string(),
                notification_type: NotificationType::Error,
            }));
        }
    };

    create_effect(move |_| match update_user_action.value()() {
        None => {
            set_update_error(Some(NotificationDetails {
                message: "Password is blank".to_string(),
                notification_type: NotificationType::Warning,
            }));
        }
        Some(Err(message)) => {
            set_update_error(Some(NotificationDetails {
                message: message.to_string(),
                notification_type: NotificationType::Error,
            }));
        }
        Some(Ok(_)) => {
            let navigate = use_navigate();
            navigate("/login", Default::default())
        }
    });

    let notification_view = move || {
        update_error().map(|details| {
            view! {
              <NotificationComponent
                notification_details=details.clone()
                on_close=move || set_update_error(None)
              />
            }
        })
    };

    view! {
      <div class="flex">
        <Sidebar />
        <div class="flex-1">
          <Suspense fallback=move || view! {}>
            <Header text="User Settings".to_string() logo=None class_id=Signal::derive(|| None) />
          </Suspense>
          <div class="p-6 mx-auto mt-8 max-w-2xl bg-white rounded-lg shadow-md user-settings">
            <h2 class="mb-6 text-2xl font-bold text-gray-800">Account Settings</h2>
            <form class="space-y-6" on:submit=on_submit>
              <div class="flex flex-col">
                <label for="name" class="mb-2 text-sm font-semibold text-gray-700">
                  Account Name
                </label>
                <input
                  type="text"
                  id="name"
                  name="name"
                  class="p-3 rounded-lg border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none"
                  prop:value=user_name
                  on:input=move |e| set_user_name(
                    e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value(),
                  )
                />
              </div>
              <div class="flex flex-col">
                <label for="password" class="mb-2 text-sm font-semibold text-gray-700">
                  New Password
                </label>
                <input
                  type="password"
                  id="password"
                  name="password"
                  class="p-3 rounded-lg border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none"
                  prop:value=password
                  on:input=move |e| set_password(
                    e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value(),
                  )
                  placeholder="Enter your new password"
                />
              </div>
              <div class="flex flex-col">
                <label for="confirm-password" class="mb-2 text-sm font-semibold text-gray-700">
                  Confirm New Password
                </label>
                <input
                  type="password"
                  id="confirm-password"
                  name="confirm-password"
                  class="p-3 rounded-lg border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none"
                  prop:value=confirm_password
                  on:input=move |e| set_confirm_password(
                    e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value(),
                  )
                  placeholder="Confirm your new password"
                />
              </div>
              <div class="flex justify-end mt-6">
                <button
                  type="submit"
                  class="py-3 px-6 font-semibold text-white bg-blue-600 rounded-lg shadow hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:outline-none"
                >
                  Save Changes
                </button>
              </div>
              {notification_view}
            </form>
          </div>
        </div>
      </div>
    }.into_view()
}
