use crate::data::database::class_functions::get_class_list;
use crate::data::database::user_functions::get_users;
use crate::pages::global_components::header::Header;
use leptos::*;
use leptos::{component, create_resource, view, For, IntoView, Signal};
use leptos_router::A;

#[component]
pub fn AdminHomePage() -> impl IntoView {
    let users = create_resource(|| {}, |_| async { get_users().await.unwrap_or_default() });
    let classes = create_resource(
        || {},
        |_| async { get_class_list().await.unwrap_or_default() },
    );

    let (is_visible, set_is_visible) = create_signal(false);

    view! {
      <Header text="ENCAMPUS".to_string() logo=None class_id=Signal::derive(|| None) />
      <div class="flex mx-6 mt-6 space-x-4">
        <div class="w-1/2">
          <div class="p-6 bg-white rounded-lg shadow-md">
            <div class="flex justify-between items-center">
              <h2 class="mb-4 text-lg font-semibold">"Users"</h2>
              <button
                class="py-1 px-2 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue"
                on:click=move |_| set_is_visible(!is_visible())
              >
                "Create User"
              </button>
            </div>

            <div class="grid grid-cols-3 gap-4">
              <div class="font-semibold">"Name"</div>
              <div class="font-semibold">"Username"</div>
              <div class="font-semibold">"Role"</div>
            </div>

            <div class="mt-4 space-y-2">
              <For each=move || users().unwrap_or_default() key=|user| user.id let:user>
                <div class="grid grid-cols-3 gap-4 p-2 border-b border-gray-200">
                  <div>{user.firstname} " " {user.lastname}</div>
                  <div>{user.username}</div>
                  <div>{user.role}</div>
                </div>
              </For>

            </div>
          </div>
        </div>

        <div class="w-1/2">
          <div class="p-6 bg-white rounded-lg shadow-md">
            <div class="flex justify-between items-center">
              <h2 class="mb-4 text-lg font-semibold">"Open Classes"</h2>
              <button
                class="py-1 px-2 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue"
                on:click=move |_| set_is_visible(!is_visible())
              >
                "Create Class"
              </button>
            </div>

            <div class="grid grid-cols-2 gap-4">
              <div class="font-semibold">"Course Name"</div>
              <div class="font-semibold">"Instructor"</div>
            </div>

            <div class="mt-4 space-y-2"></div>
            <For each=move || classes().unwrap_or_default() key=|class| class.id let:class>
              <div class="grid grid-cols-2 gap-4 p-2 border-b border-gray-200">
                <A
                  href=format!("/classes/{}", class.id)
                  class="text-blue-500 underline hover:text-blue-700"
                >
                  {class.name}
                </A>
                <div>{class.instructor}</div>
              </div>
            </For>
          </div>
        </div>

      </div>
    }
}

#[component]
fn AddUserOptions() -> impl IntoView {
    view! { <div class="p-6 bg-white rounded-lg shadow-md"></div> }
}
