use std::iter;

use crate::data::database::class_functions::{
    add_student_to_class, get_class_list, get_students_classes, ClassInfo,
};
use crate::data::database::user_functions::{add_user, get_users, update_user, User};
use crate::pages::global_components::header::Header;
use leptos::ev::Event;
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
    let(user_options_visible, set_user_options_visible) = create_signal(false);
    let(display_user, set_display_user) = create_signal(User{username: "".to_string(), firstname: "".to_string(), lastname: "".to_string(), id: 0, role: "Student".to_string(),});

    view! {
      <Header text="ENCAMPUS".to_string() logo=None class_id=Signal::derive(|| None) />
      <div class="mx-6 mt-6 space-x-4">
        <Show when=move || user_options_visible.get() fallback=|| ()>
          <UserOptions user=display_user.get()/>
        </Show>
        <Show when=move || new_user_visible.get() fallback=|| ()>
          <AddNewUser />
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
              <button class="py-1 px-2 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue">
                // on:click=move |_| set_is_visible(!is_visible())
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
fn UserOptions(user: User) -> impl IntoView {
    let (first_name_editable, set_first_name_editable) = create_signal(false);
    let (first_name, set_first_name) = create_signal(user.firstname.clone());
    let (last_name_editable, set_last_name_editable) = create_signal(false);
    let (last_name, set_last_name) = create_signal(user.lastname.clone());
    let (username_editable, set_username_editable) = create_signal(false);
    let (username, set_username) = create_signal(user.username.clone());
    let (role_editable, set_role_editable) = create_signal(false);
    let (role, set_role) = create_signal(user.role.clone());

    let all_classes = create_resource(
        || {},
        |_| async { get_class_list().await.unwrap_or_default() },
    );
    let students_classes = create_resource(
        move || user.id,
        |user_id| async move { get_students_classes(user_id).await.unwrap_or_default() },
    );

    let on_input = |setter: WriteSignal<String>| {
        move |ev| {
            setter(event_target_value(&ev));
        }
    };

    let update_user_action = create_action(move |user: &User| {
        let user = user.clone();
        async move {
            update_user(User {
                username: username.get(),
                firstname: first_name.get(),
                lastname: last_name.get(),
                id: user.id,
                role: role.get(),
            })
            .await
            .unwrap();
        }
    });

    let update_user_classes_action = create_action(move |class: &ClassInfo| {
        let class = class.clone();
        async move {
            add_student_to_class(class.id, user.id).await.unwrap();
        }
    });

    view! {
      <div class="p-6 bg-white rounded-lg shadow-md">
        <h2 class="mb-4 text-lg font-semibold">"User Options"</h2>
        <div class="grid grid-cols-2 gap-2">

          <div class="grid grid-cols-1 gap-2">
            <div class="font-semibold">"First Name"</div>
            <div class="flex items-center">
              <input
                class="p-2 rounded border"
                type="text"
                value=user.firstname
                readonly=move || !first_name_editable()
                on:input=on_input(set_first_name)
              />
              <div
                class="ml-2 text-sm text-gray-500 cursor-pointer"
                on:click=move |_| set_first_name_editable.update(|editable| *editable = !*editable)
              >
                {if first_name_editable() { "Save" } else { "Edit" }}
              </div>
            </div>
            <div class="flex items-center">
              <input
                class="p-2 rounded border"
                type="text"
                value=user.lastname
                readonly=move || !last_name_editable()
                on:input=on_input(set_last_name)
              />
              <div
                class="ml-2 text-sm text-gray-500 cursor-pointer"
                on:click=move |_| set_last_name_editable.update(|editable| *editable = !*editable)
              >
                {if last_name_editable() { "Save" } else { "Edit" }}
              </div>
            </div>
            <div class="flex items-center">
              <input
                class="p-2 rounded border"
                type="text"
                value=user.username
                readonly=move || !username_editable()
                on:input=on_input(set_username)
              />
              <div
                class="ml-2 text-sm text-gray-500 cursor-pointer"
                on:click=move |_| set_username_editable.update(|editable| *editable = !*editable)
              >
                {if username_editable() { "Save" } else { "Edit" }}
              </div>
            </div>
            <div class="flex items-center">
              <select
                class="block py-2 px-3 mt-1 w-full rounded-md border border-gray-300 shadow-sm sm:text-sm focus:border-indigo-500 focus:ring-indigo-500 focus:outline-none"
                on:change=on_input(set_role)
                prop:value=role
                readonly=move || !role_editable()
              >
                <option>"Student"</option>
                <option>"Instructor"</option>
                <option>"Admin"</option>
              </select>
              <div
                class="ml-2 text-sm text-gray-500 cursor-pointer"
                on:click=move |_| set_role_editable.update(|editable| *editable = !*editable)
              >
                {if role_editable() { "Save" } else { "Edit" }}
              </div>
            </div>
          </div>

          <div class="grid grid-cols-1 gap-2">
            <h2 class="font-semibold">"Classes"</h2>
            <ul>
              <For each=move || all_classes().unwrap_or_default() key=|class| class.id let:class>
                <li class="flex items-center">
                  <span>{class.name.clone()}</span>
                  <input
                    type="checkbox"
                    class="ml-2"
                    checked=move || {
                      if let Some(classes) = students_classes.get() {
                        classes.iter().any(|c| c.id == class.id)
                      } else {
                        false
                      }
                    }
                    on:change=move |event| {
                      let checked = event_target_checked(&event);
                      let class = class.clone();
                      if checked {
                        update_user_classes_action.dispatch(class);
                      }
                    }
                  />
                </li>
              </For>
            </ul>

          </div>
        </div>
        <div class="mt-4 text-right">
          <button
            class="py-1 px-2 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue"
            on:click=move |_| {
              update_user_action
                .dispatch(User {
                  username: username.get(),
                  firstname: first_name.get(),
                  lastname: last_name.get(),
                  role: role.get(),
                  id: user.id,
                });
            }
          >
            "Submit"
          </button>
        </div>

      </div>
    }
}

#[component]
fn AddNewUser() -> impl IntoView {
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
              class="block py-2 px-3 mt-1 w-full rounded-md border border-gray-300 shadow-sm sm:text-sm focus:border-indigo-500 focus:ring-indigo-500 focus:outline-none"
              on:input=on_input(set_first_name)
              prop:value=first_name
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">"Last Name"</label>
            <input
              type="text"
              class="block py-2 px-3 mt-1 w-full rounded-md border border-gray-300 shadow-sm sm:text-sm focus:border-indigo-500 focus:ring-indigo-500 focus:outline-none"
              on:input=on_input(set_last_name)
              prop:value=last_name
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">"Username"</label>
            <input
              type="text"
              class="block py-2 px-3 mt-1 w-full rounded-md border border-gray-300 shadow-sm sm:text-sm focus:border-indigo-500 focus:ring-indigo-500 focus:outline-none"
              on:input=on_input(set_username)
              prop:value=username
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">"Role"</label>
            <select
              class="block py-2 px-3 mt-1 w-full rounded-md border border-gray-300 shadow-sm sm:text-sm focus:border-indigo-500 focus:ring-indigo-500 focus:outline-none"
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
            on:click=move |_| {
              add_user_action
                .dispatch(User {
                  username: username.get(),
                  firstname: first_name.get(),
                  lastname: last_name.get(),
                  role: role.get(),
                  id: 0,
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
