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
          <div class="user-settings">
            <h1>{"User Settings"}</h1>
            <form>
              // Username Field
              <div class="form-group">
                <label for="username">{"Username"}</label>
                <input type="text" id="username" name="username" class="form-control" />
              </div>

              // Email Field
              <div class="form-group">
                <label for="email">{"Email"}</label>
                <input type="email" id="email" name="email" class="form-control" />
              </div>

              // Password Field
              <div class="form-group">
                <label for="password">{"Password"}</label>
                <input type="password" id="password" name="password" class="form-control" />
              </div>

              // Notification Preferences
              <div class="form-group">
                <label for="notifications">{"Notification Preferences"}</label>
                <select id="notifications" name="notifications" class="form-control">
                  <option value="all">{"All Notifications"}</option>
                  <option value="important">{"Only Important Notifications"}</option>
                  <option value="none">{"No Notifications"}</option>
                </select>
              </div>

              // Save Changes Button
              <div class="form-actions">
                <button type="submit" class="btn btn-primary">
                  {"Save Changes"}
                </button>
              </div>
            </form>
          </div>
        </div>
      </div>
    }
}
