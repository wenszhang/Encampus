use std::collections::HashMap;
use std::rc::Rc;

use crate::data::database::class_functions::{
    add_class, add_student_to_class, get_class_list, get_students_classes,
    remove_student_from_class, update_class_info, ClassInfo,
};
use crate::data::database::user_functions::{
    add_user, delete_user, get_users, get_users_by_role, update_user, User,
};
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
    let (user_options_visible, set_user_options_visible) = create_signal(false);
    let (display_user, set_display_user) = create_signal(User {
        username: "".to_string(),
        firstname: "".to_string(),
        lastname: "".to_string(),
        id: 0,
        role: "Student".to_string(),
    });
    let (display_add_class, set_display_add_class) = create_signal(false);
    let (display_class_options, set_display_class_options) = create_signal(false);
    let (display_class, set_display_class) = create_signal(ClassInfo {
        id: 0,
        name: "".to_string(),
        instructor_id: 0,
        instructor_name: "".to_string(),
    });

    view! {
      <Header text="ENCAMPUS".to_string() logo=None class_id=Signal::derive(|| None) />
      <div class="mx-6 mt-6 space-x-4">
        <Show when=move || display_class_options.get() fallback=|| ()>
          <ClassOptions class=display_class() />
        </Show>
        <Show when=move || user_options_visible.get() fallback=|| ()>
          <UserOptions user=display_user() set_user_options_visible=set_user_options_visible />
        </Show>
        <Show when=move || new_user_visible.get() fallback=|| ()>
          <AddNewUser
            this_window_open=set_new_user_visible
            show_user_options=set_user_options_visible
            display_user=set_display_user
          />
        </Show>
        <Show when=move || display_add_class.get() fallback=|| ()>
          <AddClass />
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
                {
                  let cloned_user = user.clone();
                  view! {
                    <div class="grid grid-cols-3 gap-4 p-2 border-b border-gray-200">
                      <a
                        href="#"
                        class="text-blue-600 underline"
                        on:click=move |_| {
                          set_display_user(cloned_user.clone());
                          set_user_options_visible(!user_options_visible());
                          set_new_user_visible(false);
                        }
                      >
                        {user.firstname}
                        " "
                        {user.lastname}
                      </a>
                      <div>{user.username}</div>
                      <div>{user.role}</div>
                    </div>
                  }
                }
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
                on:click=move |_| set_display_add_class(!display_add_class())
              >
                "Create Class"
              </button>
            </div>

            <div class="grid grid-cols-3 gap-4">
              <div class="font-semibold">"Course Name"</div>
              <div class="font-semibold">"Instructor"</div>
            </div>

            <div class="mt-4 space-y-2"></div>
            <For each=move || classes().unwrap_or_default() key=|class| class.id let:class>
              {
                let class_clone = class.clone();
                view! {
                  <div class="grid grid-cols-3 gap-4 p-2 border-b border-gray-200">
                    <A
                      href=format!("/classes/{}", class.id.clone())
                      class="text-blue-500 underline hover:text-blue-700"
                    >
                      {class.name.clone()}
                    </A>
                    <div>{class.instructor_name.clone()}</div>
                    <button
                      class="py-1 px-2 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue"
                      on:click=move |_| {
                        set_display_class(class_clone.clone());
                        set_display_class_options(!display_class_options());
                      }
                    >
                      "Class Options"
                    </button>
                  </div>
                }
              }
            </For>
          </div>
        </div>

      </div>
    }
}

#[component]
fn UserOptions(user: User, set_user_options_visible: WriteSignal<bool>) -> impl IntoView {
    let (first_name_editable, set_first_name_editable) = create_signal(false);
    let (first_name, set_first_name) = create_signal(user.firstname.clone());
    let (last_name_editable, set_last_name_editable) = create_signal(false);
    let (last_name, set_last_name) = create_signal(user.lastname.clone());
    let (username_editable, set_username_editable) = create_signal(false);
    let (username, set_username) = create_signal(user.username.clone());
    let (role_editable, set_role_editable) = create_signal(false);
    let (role, set_role) = create_signal(user.role.clone());
    let (user, set_user) = create_signal(user.clone());

    let input_user = user.clone();
    let user_delete = user.clone();
    let user = Rc::new(user);

    let (update_info, set_update_info) = create_signal(false);

    let all_classes = create_resource(
        || {},
        |_| async { get_class_list().await.unwrap_or_default() },
    );
    let students_classes = create_resource(|| {}, {
        let user = Rc::clone(&user);
        move |_| {
            let user = user.clone();
            async move {
                get_students_classes(user.get().id)
                    .await
                    .unwrap_or_default()
            }
        }
    });

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

    let delete_user_action = create_action({
        let user = user.clone();
        move |user: &User| {
            let user = user.clone();
            async move {
                delete_user(user.clone()).await.unwrap_or_default();
                set_user_options_visible(false);
                let navigate = leptos_router::use_navigate();
                navigate("/AdminHomePage", Default::default())
            }
        }
    });

    let (class_selections, set_class_selections) = create_signal(HashMap::new());

    let add_user_classes_action = create_action({
        let user = user.clone();
        move |(class_id, user): &(i32, User)| {
            let class_id = class_id.clone();
            let user = user.clone();
            async move {
                add_student_to_class(class_id, user.id).await.unwrap();
            }
        }
    });

    let remove_user_from_class_action = create_action({
        let user = user.clone();
        move |(class_id, user): &(i32, User)| {
            let class_id = *class_id;
            let user = user.clone();
            async move {
                remove_student_from_class(class_id, user.id).await.unwrap();
            }
        }
    });

    view! {
      <div class="p-6 bg-white rounded-lg shadow-md">
        <div class="flex justify-between items-start mb-4">
          <h2 class="mb-4 text-lg font-semibold">"User Options"</h2>
          <button
            class="py-1 px-2 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue"
            on:click=move |_| {
              let user_delete = user_delete.clone();
              delete_user_action.dispatch(user_delete.get());
            }
          >
            "Delete User"
          </button>
        </div>
        <div class="grid grid-cols-2 gap-2">

          <div class="grid grid-cols-1 gap-2">
            <div class="font-semibold">"First Name"</div>
            <div class="flex items-center">
              <input
                class="p-2 rounded border"
                type="text"
                value=input_user.get().firstname
                readonly=move || !first_name_editable()
                on:input=move |ev| {
                  set_first_name(event_target_value(&ev));
                  set_update_info(true);
                }
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
                value=input_user.get().lastname
                readonly=move || !last_name_editable()
                on:input=move |ev| {
                  set_last_name(event_target_value(&ev));
                  set_update_info(true);
                }
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
                value=input_user.get().username
                readonly=move || !username_editable()
                on:input=move |ev| {
                  set_username(event_target_value(&ev));
                  set_update_info(true)
                }
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
                on:change=move |ev| {
                  let new_value = event_target_value(&ev);
                  set_role(new_value);
                }
                prop:value=move || role.get()
                readonly=move || !role_editable()
              >
                <option value="Student">"Student"</option>
                <option value="Instructor">"Instructor"</option>
                <option value="Admin">"Admin"</option>
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
                      set_class_selections
                        .update(move |selections| {
                          selections.insert(class.id, checked);
                        });
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
              if update_info() {
                update_user_action
                  .dispatch(User {
                    username: username.get(),
                    firstname: first_name.get(),
                    lastname: last_name.get(),
                    role: role.get(),
                    id: input_user.get().id,
                  });
              }
              for (class_id, selected) in class_selections.get().iter() {
                if *selected {
                  add_user_classes_action.dispatch((*class_id, user.get()));
                } else {
                  remove_user_from_class_action.dispatch((*class_id, user_delete.get()));
                }
              }
            }
          >
            "Submit"
          </button>
        </div>

      </div>
    }
}

#[component]
fn AddNewUser(
    this_window_open: WriteSignal<bool>,
    show_user_options: WriteSignal<bool>,
    display_user: WriteSignal<User>,
) -> impl IntoView {
    let (first_name, set_first_name) = create_signal("".to_string());
    let (last_name, set_last_name) = create_signal("".to_string());
    let (username, set_username) = create_signal("".to_string());
    let (role, set_role) = create_signal("".to_string());

    let on_input = |setter: WriteSignal<String>| {
        move |ev| {
            setter(event_target_value(&ev));
        }
    };

    let add_user_action = create_action(move |user: &User| {
        let user = user.clone();
        async move {
            let user = add_user(user, "password".to_string()).await.unwrap();
            display_user(user);
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
              on:change=move |ev| {
                let new_value = event_target_value(&ev);
                set_role(new_value);
              }
              prop:value=move || role.get()
            >
              <option value="student">"Student"</option>
              <option value="instructor">"Instructor"</option>
              <option value="admin">"Admin"</option>
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
              this_window_open.update(|value| *value = !*value);
              show_user_options.update(|value2| *value2 = !*value2);
            }
          >
            "Submit"
          </button>
        </div>
      </div>
    }
}

#[component]
fn AddClass() -> impl IntoView {
    let (class_name, set_class_name) = create_signal("".to_string());
    let (instructor_id, set_instructor_id) = create_signal(0);

    let instructors = create_resource(
        || {},
        |_| async {
            get_users_by_role("instructor".to_string())
                .await
                .unwrap_or_default()
        },
    );

    let add_class_action = create_action(move |_| {
        // let class = class.clone();
        async move {
            add_class(class_name(), instructor_id()).await.unwrap();
        }
    });

    let on_input = |setter: WriteSignal<String>| {
        move |ev| {
            setter(event_target_value(&ev));
        }
    };

    view! {
      <div class="p-6 bg-white rounded-lg shadow-md">
        <h2 class="mb-4 text-lg font-semibold">"New Class"</h2>
        <div class="grid grid-cols-1 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700">"Class Name"</label>
            <input
              type="text"
              class="block py-2 px-3 mt-1 w-full rounded-md border border-gray-300 shadow-sm sm:text-sm focus:border-indigo-500 focus:ring-indigo-500 focus:outline-none"
              on:input=on_input(set_class_name)
              prop:value=class_name
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">"Instructor"</label>
            <select
              class="block py-2 px-3 mt-1 w-full rounded-md border border-gray-300 shadow-sm sm:text-sm focus:border-indigo-500 focus:ring-indigo-500 focus:outline-none"
              on:change=move |ev| {
                let new_value = event_target_value(&ev);
                set_instructor_id(new_value.parse().unwrap());
              }
              prop:value=move || instructor_id.get()
            >
              <For
                each=move || instructors().unwrap_or_default()
                key=|current_instructor| current_instructor.id
                let:current_instructor
              >
                <option value=current_instructor
                  .id>{current_instructor.firstname} " " {current_instructor.lastname}</option>
              </For>
            </select>
          </div>
        </div>
        <div class="mt-4 text-right">
          <button
            class="py-1 px-2 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue"
            on:click=move |_| {
              add_class_action.dispatch(|| {});
            }
          >
            "Submit"
          </button>
        </div>
      </div>
    }
}

#[component]
fn ClassOptions(class: ClassInfo) -> impl IntoView {
    let (class_name, set_class_name) = create_signal(class.name.clone());
    let (class_name_editable, set_class_name_editable) = create_signal(false);
    let (instructor_id, set_instructor_id) = create_signal(class.instructor_id);
    let (instructor_name_editable, set_instructor_name_editable) = create_signal(false);

    let update_class_action = create_action(move |class: &ClassInfo| {
        let class = class.clone();
        async move {
            update_class_info(class, instructor_id()).await.unwrap();
        }
    });

    let instructors = create_resource(
        || {},
        |_| async {
            get_users_by_role("instructor".to_string())
                .await
                .unwrap_or_default()
        },
    );

    view! {
      <div class="p-6 bg-white rounded-lg shadow-md">
        <h2 class="mb-4 text-lg font-semibold">"Class Options"</h2>
        <div class="grid grid-cols-1 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700">"Class Name"</label>
            <input
              type="text"
              class="block py-2 px-3 mt-1 w-full rounded-md border border-gray-300 shadow-sm sm:text-sm focus:border-indigo-500 focus:ring-indigo-500 focus:outline-none"
              prop:value=class.name
              readonly=move || !class_name_editable()
              on:input=move |ev| {
                set_class_name(event_target_value(&ev));
              }
            />
            <div
              class="ml-2 text-sm text-gray-500 cursor-pointer"
              on:click=move |_| set_class_name_editable.update(|editable| *editable = !*editable)
            >
              {if class_name_editable() { "Save" } else { "Edit" }}
            </div>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">"Instructor"</label>
            <select
              class="block py-2 px-3 mt-1 w-full rounded-md border border-gray-300 shadow-sm sm:text-sm focus:border-indigo-500 focus:ring-indigo-500 focus:outline-none"
              disabled=move || !instructor_name_editable()
              on:input=move |ev| {
                set_instructor_id(event_target_value(&ev).parse().unwrap());
              }
              prop:value=move || instructor_id.get()
            >
              <For
                each=move || instructors().unwrap_or_default()
                key=|current_instructor| current_instructor.id
                let:current_instructor
              >
                <option value=current_instructor
                  .id>{current_instructor.firstname} " " {current_instructor.lastname}</option>
              </For>
            </select>
            <div
              class="ml-2 text-sm text-gray-500 cursor-pointer"
              on:click=move |_| {
                set_instructor_name_editable.update(|editable| *editable = !*editable)
              }
            >
              {if instructor_name_editable() { "Save" } else { "Edit" }}
            </div>
          </div>

        </div>

        <div class="mt-4 text-right">
          <button
            class="py-1 px-2 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue"
            on:click=move |_| {
              update_class_action
                .dispatch(ClassInfo {
                  id: class.id,
                  name: class_name.get(),
                  instructor_id: instructor_id.get(),
                  instructor_name: instructors
                    .get()
                    .unwrap()
                    .iter()
                    .find(|i| i.id == instructor_id.get())
                    .unwrap()
                    .firstname
                    .clone(),
                });
            }
          >
            "Submit"
          </button>
        </div>
      </div>
    }
}
