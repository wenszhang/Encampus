use crate::data::database::user_functions::update_user_credentials;
use crate::pages::global_components::sidebar::Sidebar;
use crate::{data::global_state::GlobalState, pages::global_components::header::Header};
use leptos::ev::SubmitEvent;
use leptos::{
    component, expect_context, view, IntoView, Signal, SignalGetUntracked, SignalSet, Suspense,
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlFormElement, HtmlInputElement};

/// Renders the user settings page
#[component]
pub fn UserSettings() -> impl IntoView {
    let global_state = expect_context::<GlobalState>();
    let user_id = global_state.id.get_untracked().unwrap_or_default();
    let user_name = global_state.user_name.get_untracked().unwrap_or_default();

    // Get form data
    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();

        let form = event
            .target()
            .unwrap()
            .dyn_into::<HtmlFormElement>()
            .unwrap();

        let name = form
            .elements()
            .named_item("name")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();

        let password = form
            .elements()
            .named_item("password")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();

        let confirm_password = form
            .elements()
            .named_item("confirm-password")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();

        if password == confirm_password {
            let name_clone = name.clone();
            let password_clone = password.clone();

            // Call the async function to update user credentials
            let future = async move {
                match update_user_credentials(user_id, name_clone, password_clone).await {
                    Ok(_) => {
                        global_state.user_name.set(Some(name.clone()));
                        // Show success
                    }
                    Err(_) => {
                        // Handle the error
                    }
                }
            };
            leptos::spawn_local(future);
        } else {
            // Handle password mismatch (e.g., show an error message)
        }
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
                  Account ID
                </label>
                <input
                  type="text"
                  id="name"
                  name="name"
                  class="p-3 rounded-lg border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none"
                  value=user_name
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
            </form>
          </div>
        </div>
      </div>
    }
}
