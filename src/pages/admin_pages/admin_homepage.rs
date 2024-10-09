use crate::data::database::class_functions::get_class_list;
use crate::data::database::user_functions::get_users;
use crate::data::database::user_functions::User;
use crate::data::database::user_functions::add_user;
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

    let (new_user_visible, set_new_user_visible) = create_signal(false);

    view! {
      <Header text="ENCAMPUS".to_string() logo=None class_id=Signal::derive(|| None) />
      <div class="mt-6 mx-6 space-x-4">
        <Show when=move || new_user_visible.get() fallback=|| ()>
          <AddUserOptions/>
        </Show>
      </div>
      
      <div class="flex mx-6 mt-6 space-x-4">
        <div class="w-1/2">
          <div class="p-6 bg-white rounded-lg shadow-md">
            <div class="flex justify-between items-center">
              <h2 class="mb-4 text-lg font-semibold">"Users"</h2>
              <button
                class="py-1 px-2 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue"
                on:click=move |_| set_new_user_visible(!new_user_visible())
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
                // on:click=move |_| set_is_visible(!is_visible())
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
    let (first_name, set_first_name) = create_signal("".to_string());
    let (last_name, set_last_name) = create_signal("".to_string());
    let (username, set_username) = create_signal("".to_string());
    let (role, set_role) = create_signal("Student".to_string());

    let on_input = |setter: WriteSignal<String>| {
      move |ev| {
          setter(event_target_value(&ev));
      }
  };

    let add_user_action = create_action(move |user: &User| {
      let user = user.clone();
      async move {
        add_user(user).await.unwrap();
      }
    });
    view! { 
      <div class="p-6 bg-white rounded-lg shadow-md">
        <h2 class="mb-4 text-lg font-semibold">"New User"</h2>
        <div class="grid grid-cols-1 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700">"First Name"</label>
            <input
              type="text"
              class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
              on:input=on_input(set_first_name)
              prop:value=first_name
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">"Last Name"</label>
            <input
              type="text"
              class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
              on:input=on_input(set_last_name)  
              prop:value=last_name
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">"Username"</label>
            <input
              type="text"
              class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
              on:input=on_input(set_username) 
              prop:value=username
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">"Role"</label>
            <select
              class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
              on:change=on_input(set_role)
              prop:value=role
            >
              <option>"Student"</option>
              <option>"Instructor"</option>
              <option>"Admin"</option>
            </select>
          </div>
        </div>
        <div class="mt-4 text-right">
          <button
            class="py-1 px-2 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue"
            on:click=move |_| {add_user_action.dispatch(User {
              username: username.get(),
              firstname: first_name.get(),
              lastname: last_name.get(),
              role: role.get(),
              id: 0, // Will get overwritten when inserted, but required for struct
            });
            set_first_name("".to_string());
            set_last_name("".to_string());
            set_username("".to_string());
            set_role("Student".to_string());
          }
          >
            "Submit"
          </button>
        </div>
      </div>
    }
}
