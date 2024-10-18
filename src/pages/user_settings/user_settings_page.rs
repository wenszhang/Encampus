use crate::data::database::user_functions::{get_user_by_id, update_user, User};
use crate::pages::global_components::sidebar::Sidebar;
use crate::{data::global_state::GlobalState, pages::global_components::header::Header};
use leptos::ev::SubmitEvent;
use leptos::{
    component, create_signal, expect_context, view, IntoView, Signal, SignalGet,
    SignalGetUntracked, SignalSet, Suspense,
};
use wasm_bindgen::JsCast;
use web_sys::window;
use web_sys::{HtmlFormElement, HtmlInputElement};

/// Renders the user settings page
#[component]
pub fn UserSettings() -> impl IntoView {
    let global_state = expect_context::<GlobalState>();
    let user_id = global_state.id.get_untracked().unwrap_or_default();
    let (user_name, set_user_name) =
        create_signal(global_state.user_name.get_untracked().unwrap_or_default());
    let (password, set_password) = create_signal(String::new());
    let (confirm_password, set_confirm_password) = create_signal(String::new());

    // Get form data
    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();

        let user_id = user_id.clone();
        let user_name = user_name.get();
        let password = password.get();
        let confirm_password = confirm_password.get();

        if password == confirm_password {
            let future = async move {
                match get_user_by_id(user_id).await {
                    Ok(mut user) => {
                        user.username = user_name.clone();

                        match update_user(user).await {
                            Ok(_) => {
                                global_state.user_name.set(Some(user_name));
                                window().unwrap().location().set_href("/login").unwrap();
                            }
                            Err(_) => {
                                // Handle the error
                            }
                        }
                    }
                    Err(_) => {
                        // Handle the error when getting the user fails
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
          <Suspense fallback=move || view! { }>
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
                  on:input=move |e| set_user_name(e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value())
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
                  on:input=move |e| set_password(e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value())
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
                  on:input=move |e| set_confirm_password(e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value())
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
