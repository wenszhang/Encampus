use crate::pages::global_components::sidebar::Sidebar;
use crate::{data::global_state::GlobalState, pages::global_components::header::Header};
/** Page for users to manage their account settings */
use leptos::{component, expect_context, view, IntoView, Signal, Suspense};

/// Renders the user settings page
#[component]
pub fn UserSettings() -> impl IntoView {
    let global_state = expect_context::<GlobalState>();

    view! {
      <div class="flex">
        <Sidebar />
        <div class="flex-1">
          <Suspense fallback=move || view! {}>
            <Header text="User Settings".to_string() logo=None class_id=Signal::derive(|| None) />
          </Suspense>
          <div class="p-6 mx-auto mt-8 max-w-2xl bg-white rounded-lg shadow-md user-settings">
            <h2 class="mb-6 text-2xl font-bold text-gray-800">Account Settings</h2>
            <form class="space-y-6">
              <div class="flex flex-col">
                <label for="name" class="mb-2 text-sm font-semibold text-gray-700">
                  Account Name
                </label>
                <input
                  type="text"
                  id="name"
                  name="name"
                  class="p-3 rounded-lg border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none"
                  placeholder="Enter your name"
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
