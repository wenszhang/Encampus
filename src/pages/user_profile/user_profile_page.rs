use crate::data::database::user_functions::update_user;
use crate::pages::global_components::sidebar::Sidebar;
use crate::{data::global_state::GlobalState, pages::global_components::header::Header};
use leptos::ev::SubmitEvent;
use leptos::{component, expect_context, view, IntoView, Signal, SignalGetUntracked, Suspense};

/// Renders the user settings page
#[component]
pub fn UserProfile() -> impl IntoView {
    let global_state = expect_context::<GlobalState>();
    let user_id = global_state.id.get_untracked().unwrap_or_default();
    let first_name = global_state.first_name.get_untracked().unwrap_or_default();
    let last_name = global_state.last_name.get_untracked().unwrap_or_default();

    let on_submit = move |event: SubmitEvent| {};

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
              <div class="flex justify-center items-center w-24 h-24 bg-gray-300 rounded-full">
                <span class="text-gray-500">P</span>
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
                  value=first_name
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
                  value=last_name
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
