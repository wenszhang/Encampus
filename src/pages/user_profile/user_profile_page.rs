use crate::data::database::user_functions::update_user_without_password;
use crate::data::global_state::{User, UserBuilder};
use crate::pages::global_components::header::Header;
use crate::pages::global_components::notification::{
    NotificationComponent, NotificationDetails, NotificationType,
};
use crate::pages::global_components::sidebar::Sidebar;
use crate::{expect_logged_in_user, on_input};
use leptos::ev::SubmitEvent;
use leptos::{component, create_action, create_signal, view, IntoView, Signal, Suspense};
use leptos::{create_effect, SignalGetUntracked};

/// Renders the user settings page
#[component]
pub fn UserProfile() -> impl IntoView {
    let (user, set_user) = expect_logged_in_user!();
    let (first_name, set_first_name) = create_signal(user.get_untracked().first_name.clone());
    let (last_name, set_last_name) = create_signal(user.get_untracked().last_name.clone());
    let (update_error, set_update_error) = create_signal(None::<NotificationDetails>);

    let update_user_action = create_action(move |updated_user: &User| {
        let updated_user = updated_user.clone();
        async move {
            update_user_without_password(updated_user)
                .await
                .map(|dbUser| User {
                    id: dbUser.id,
                    first_name: dbUser.firstname,
                    last_name: dbUser.lastname,
                    user_name: dbUser.username,
                    role: dbUser.role,
                })
                .map_err(|_server_fn_err| "An error occurred. Please try again")
        }
    });

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        let mut user = user.get_untracked();
        user.first_name = first_name.get_untracked().clone();
        user.last_name = last_name.get_untracked().clone();
        update_user_action.dispatch(user);
    };

    create_effect(move |_| match update_user_action.value()() {
        None => {}
        Some(Err(message)) => {
            set_update_error(Some(NotificationDetails {
                message: message.to_string(),
                notification_type: NotificationType::Error,
            }));
        }
        Some(Ok(user)) => {
            set_user(UserBuilder {
                first_name: Some(user.first_name.clone()),
                last_name: Some(user.last_name.clone()),
                user_name: Some(user.user_name.clone()),
                id: Some(user.id.clone()),
                role: Some(user.role.clone()),
            });
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
            <Header text="User Profile".to_string() logo=None class_id=Signal::derive(|| None) />
          </Suspense>
          <div class="p-6 mx-auto mt-8 max-w-2xl bg-white rounded-lg shadow-md user-settings">
            <h2 class="mb-6 text-2xl font-bold text-gray-800">Profile Settings</h2>
            <div class="flex justify-center items-center mb-6">
              <div class="flex overflow-hidden justify-center items-center w-24 h-24 rounded-full">
                <img
                  src="https://static.vecteezy.com/system/resources/thumbnails/009/292/244/small/default-avatar-icon-of-social-media-user-vector.jpg"
                  alt="Profile Image"
                  class="object-cover w-full h-full"
                />
              </div>
            </div>
            <form class="space-y-6" on:submit=on_submit>
              <div class="flex flex-col">
                <label for="first_name" class="mb-2 text-sm font-semibold text-gray-700">
                  First Name
                </label>
                <input
                  type="text"
                  id="first_name"
                  name="first_name"
                  class="p-3 rounded-lg border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none"
                  prop:value=first_name
                  on:input=on_input!(set_first_name)
                />
              </div>
              <div class="flex flex-col">
                <label for="last_name" class="mb-2 text-sm font-semibold text-gray-700">
                  Last Name
                </label>
                <input
                  type="text"
                  id="last_name"
                  name="last_name"
                  class="p-3 rounded-lg border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none"
                  prop:value=last_name
                  on:input=on_input!(set_last_name)
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
