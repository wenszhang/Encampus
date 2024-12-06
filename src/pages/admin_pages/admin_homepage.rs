use crate::data::database::class_functions::{
    add_class, add_student_to_class, add_ta_to_class, delete_class, get_class_list, get_classes_ta,
    get_instructors_classes, get_students_classes, remove_student_from_class, remove_ta_from_class,
    update_class_info, ClassInfo,
};
use crate::data::database::user_functions::{
    add_user, delete_user, get_users, get_users_by_role, update_user, update_user_without_password,
};
use crate::data::global_state::User;
use crate::on_input;
use crate::pages::global_components::header::Header;
use crate::pages::register_page::NewUser;
use leptos::*;
use leptos::{component, create_resource, view, For, IntoView, Signal};
use leptos_router::A;
use std::collections::HashMap;

#[component]
pub fn AdminHomePage() -> impl IntoView {
    let users = create_resource(
        || {},
        |_| async {
            get_users()
                .await
                .map(|array| {
                    array
                        .iter()
                        .map(|user| User {
                            user_name: user.username.clone(),
                            id: user.id,
                            first_name: user.firstname.clone(),
                            last_name: user.lastname.clone(),
                            role: user.role.clone(),
                        })
                        .collect::<Vec<User>>()
                })
                .unwrap_or_default()
        },
    );
    let classes = create_resource(
        || {},
        |_| async { get_class_list().await.unwrap_or_default() },
    );

    let (new_user_visible, set_new_user_visible) = create_signal(false);
    let (user_options_visible, set_user_options_visible) = create_signal(false);
    let (display_user, set_display_user) = create_signal(User {
        user_name: "".to_string(),
        first_name: "".to_string(),
        last_name: "".to_string(),
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
        description: "".to_string(),
    });

    view! {
      <Header text="ENCAMPUS".to_string() logo=None class_id=Signal::derive(|| None) />
      <div class="mx-6 mt-6 space-x-4">
        <Show when=move || display_class_options.get() fallback=|| ()>
          <ClassOptions
            class=display_class()
            set_display_class_options=set_display_class_options
            display_class_options=set_display_class_options
          />
        </Show>
        <Show when=move || user_options_visible.get() fallback=|| ()>
          <UserOptions
            user=display_user()
            set_user_options_visible=set_user_options_visible
            display_user_options=set_user_options_visible
          />
        </Show>
        <Show when=move || new_user_visible.get() fallback=|| ()>
          <AddNewUser
            this_window_open=set_new_user_visible
            show_user_options=set_user_options_visible
            display_user=set_display_user
            display_add_user=set_new_user_visible
          />
        </Show>
        <Show when=move || display_add_class.get() fallback=|| ()>
          <AddClass display_add_class=set_display_add_class />
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
                          set_user_options_visible(true);
                          set_new_user_visible(false);
                        }
                      >
                        {user.first_name}
                        " "
                        {user.last_name}
                      </a>
                      <div>{user.user_name}</div>
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

            <div class="mt-4 space-y-2">
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
                          set_display_class_options(true);
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
      </div>
    }
}

#[component]
fn UserOptions(
    user: User,
    set_user_options_visible: WriteSignal<bool>,
    display_user_options: WriteSignal<bool>,
) -> impl IntoView {
    let (first_name, set_first_name) = create_signal(user.first_name.clone());
    let (last_name, set_last_name) = create_signal(user.last_name.clone());
    let (username, set_username) = create_signal(user.user_name.clone());
    let (role, set_role) = create_signal(user.role.clone());
    let (user, _set_user) = create_signal(user.clone());
    let (password, set_password) = create_signal("".to_string());
    let (edit_password, set_edit_password) = create_signal(false);

    let all_classes = create_resource(
        || {},
        |_| async { get_class_list().await.unwrap_or_default() },
    );
    let users_classes = create_resource(|| {}, {
        move |_| async move {
            if user.get().role == "Instructor" {
                get_instructors_classes(user.get().id)
                    .await
                    .unwrap_or_default()
            } else {
                get_students_classes(user.get().id)
                    .await
                    .unwrap_or_default()
            }
        }
    });

    let ta_classes = create_resource(|| {}, {
        move |_| async move { get_classes_ta(user.get().id).await.unwrap_or_default() }
    });

    let update_user_action = create_action(move |user: &User| {
        let user = user.clone();
        async move {
            if edit_password.get() {
                update_user(NewUser {
                    user: User {
                        user_name: username.get(),
                        first_name: first_name.get(),
                        last_name: last_name.get(),
                        id: user.id,
                        role: role.get(),
                    },
                    password: password.get(),
                })
                .await
                .unwrap();
            } else {
                update_user_without_password(User {
                    user_name: username.get(),
                    first_name: first_name.get(),
                    last_name: last_name.get(),
                    id: user.id,
                    role: role.get(),
                })
                .await
                .unwrap();
            }
        }
    });

    let delete_user_action = create_action({
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
        move |(class_id, user): &(i32, User)| {
            let class_id = *class_id;
            let user = user.clone();
            async move {
                add_student_to_class(class_id, user.id).await.unwrap();
            }
        }
    });

    let remove_user_from_class_action = create_action({
        move |(class_id, user): &(i32, User)| {
            let class_id = *class_id;
            let user = user.clone();
            async move {
                remove_student_from_class(class_id, user.id).await.unwrap();
            }
        }
    });

    let (ta_selections, set_ta_selections) = create_signal(HashMap::new());

    let add_user_as_ta_action = create_action({
        move |(class_id, user): &(i32, User)| {
            let class_id = *class_id;
            let user = user.clone();
            async move {
                add_ta_to_class(user.id, class_id).await.unwrap();
            }
        }
    });

    let remove_user_as_ta_action = create_action({
        move |(class_id, user): &(i32, User)| {
            let class_id = *class_id;
            let user = user.clone();
            async move {
                remove_ta_from_class(user.id, class_id).await.unwrap();
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
              display_user_options.update(|value| *value = !*value);
            }
          >
            "Close"
          </button>
        </div>
        <div class="grid grid-cols-2 gap-4">
          <div class="grid grid-cols-1 gap-2">
            <div class="font-semibold">"First Name"</div>
            <div class="flex items-center">
              <input
                class="p-2 rounded border"
                type="text"
                value=first_name
                on:input=move |ev| {
                  set_first_name(event_target_value(&ev));
                }
              />
            </div>
            <div class="flex items-center">
              <input
                class="p-2 rounded border"
                type="text"
                value=last_name
                on:input=move |ev| {
                  set_last_name(event_target_value(&ev));
                }
              />
            </div>
            <div class="flex items-center">
              <input
                class="p-2 rounded border"
                type="text"
                value=username
                on:input=move |ev| {
                  set_username(event_target_value(&ev));
                }
              />
            </div>
            <div class="flex items-center">
              <select
                class="block py-2 px-3 mt-1 w-full rounded-md border border-gray-300 shadow-sm sm:text-sm focus:border-indigo-500 focus:ring-indigo-500 focus:outline-none"
                on:input=move |ev| {
                  set_role(event_target_value(&ev));
                  set_edit_password(true);
                }
                prop:value=move || role.get().to_string()
              >
                <option role="Student">"Student"</option>
                <option role="Instructor">"Instructor"</option>
                <option role="Admin">"Admin"</option>
              </select>
            </div>
            <div class="flex items-center">
              <label class="mr-4 font-semibold">"New Password:"</label>
              <input
                class="p-2 rounded border"
                type="text"
                on:input=move |ev| {
                  set_password(event_target_value(&ev));
                  set_edit_password(true);
                }
              />
            </div>
          </div>
          {if role.get() == "Student" {
            view! {
              <div class="grid grid-cols-2 gap-4">
                <div>
                  <h2 class="font-semibold">"Classes"</h2>
                  <div class="grid grid-cols-1 gap-2">
                    <ul>
                      <For
                        each=move || all_classes().unwrap_or_default()
                        key=|class| class.id
                        let:class
                      >
                        <li class="flex items-center">
                          <span>{class.name.clone()}</span>
                          <input
                            type="checkbox"
                            class="ml-2"
                            checked=move || {
                              if let Some(classes) = users_classes.get() {
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
                <div>
                  <h2 class="font-semibold">"Enroll as TA"</h2>
                  <div class="grid grid-cols-1 gap-2">
                    <ul>
                      <For
                        each=move || all_classes().unwrap_or_default()
                        key=|class| class.id
                        let:class
                      >
                        <li class="flex items-center">
                          <span>{class.name.clone()}</span>
                          <input
                            type="checkbox"
                            class="ml-2"
                            checked=move || {
                              if let Some(classes) = ta_classes.get() {
                                classes.iter().any(|c| c.id == class.id)
                              } else {
                                false
                              }
                            }
                            on:change=move |event| {
                              let checked = event_target_checked(&event);
                              let class = class.clone();
                              set_ta_selections
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
              </div>
            }
          } else if role.get() == "Instructor" {
            view! {
              <div>
                <h2 class="mb-2 font-semibold">"Classes Teaching"</h2>
                <div class="grid grid-cols-1 gap-2">
                  <ul>
                    <For
                      each=move || users_classes().unwrap_or_default()
                      key=|class| class.id
                      let:class
                    >
                      <li class="flex items-center">{class.name.clone()}</li>
                    </For>
                  </ul>
                </div>
              </div>
            }
          } else {
            view! { <div></div> }
          }}

        </div>
        <div class="flex justify-between items-center mt-4">
          <button
            class="py-1 px-2 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue"
            on:click=move |_| {
              delete_user_action.dispatch(user.get());
            }
          >
            "Delete User"
          </button>
          <button
            class="py-1 px-2 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue"
            on:click=move |_| {
              update_user_action
                .dispatch(User {
                  user_name: username.get(),
                  first_name: first_name.get(),
                  last_name: last_name.get(),
                  role: role.get(),
                  id: user.get().id,
                });
              for (class_id, selected) in class_selections.get().iter() {
                if *selected {
                  add_user_classes_action.dispatch((*class_id, user.get()));
                } else {
                  remove_user_from_class_action.dispatch((*class_id, user.get()));
                }
              }
              for (class_id, selected) in ta_selections.get().iter() {
                if *selected {
                  add_user_as_ta_action.dispatch((*class_id, user.get()));
                } else {
                  remove_user_as_ta_action.dispatch((*class_id, user.get()));
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
    display_add_user: WriteSignal<bool>,
) -> impl IntoView {
    let (first_name, set_first_name) = create_signal("".to_string());
    let (last_name, set_last_name) = create_signal("".to_string());
    let (username, set_username) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (role, set_role) = create_signal("Student".to_string()); // Set to student by default

    let add_user_action = create_action(move |(user, password): &(User, String)| {
        let user = user.clone();
        let password = password.clone();
        async move {
            let user = add_user(
                NewUser {
                    user: user.clone(),
                    password,
                },
                false,
            )
            .await
            .map(|dnUser| User {
                user_name: dnUser.username,
                first_name: dnUser.firstname,
                last_name: dnUser.lastname,
                id: dnUser.id,
                role: dnUser.role,
            })
            .unwrap();
            display_user(user.clone());
        }
    });
    view! {
      <div class="p-6 bg-white rounded-lg shadow-md">
        <div class="flex justify-between items-start mb-4">
          <h2 class="mb-4 text-lg font-semibold">"New User"</h2>
          <button
            class="py-1 px-2 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue"
            on:click=move |_| {
              display_add_user.update(|value| *value = !*value);
            }
          >
            "Close"
          </button>
        </div>
        <div class="grid grid-cols-1 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700">"First Name"</label>
            <input
              type="text"
              class="block py-2 px-3 mt-1 w-full rounded-md border border-gray-300 shadow-sm sm:text-sm focus:border-indigo-500 focus:ring-indigo-500 focus:outline-none"
              on:input=on_input!(set_first_name)
              prop:value=first_name
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">"Last Name"</label>
            <input
              type="text"
              class="block py-2 px-3 mt-1 w-full rounded-md border border-gray-300 shadow-sm sm:text-sm focus:border-indigo-500 focus:ring-indigo-500 focus:outline-none"
              on:input=on_input!(set_last_name)
              prop:value=last_name
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">"Username"</label>
            <input
              type="text"
              class="block py-2 px-3 mt-1 w-full rounded-md border border-gray-300 shadow-sm sm:text-sm focus:border-indigo-500 focus:ring-indigo-500 focus:outline-none"
              on:input=on_input!(set_username)
              prop:value=username
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">"Password"</label>
            <input
              type="password"
              class="block py-2 px-3 mt-1 w-full rounded-md border border-gray-300 shadow-sm sm:text-sm focus:border-indigo-500 focus:ring-indigo-500 focus:outline-none"
              on:input=on_input!(set_password)
              prop:value=password
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
              <option value="Student">"Student"</option>
              <option value="Instructor">"Instructor"</option>
              <option value="Admin">"Admin"</option>
            </select>
          </div>
        </div>
        <div class="mt-4 text-right">
          <button
            class="py-1 px-2 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue"
            on:click=move |_| {
              add_user_action
                .dispatch((
                  User {
                    user_name: username.get(),
                    first_name: first_name.get(),
                    last_name: last_name.get(),
                    role: role.get(),
                    id: 0,
                  },
                  password.get(),
                ));
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
fn AddClass(display_add_class: WriteSignal<bool>) -> impl IntoView {
    let (class_name, set_class_name) = create_signal("".to_string());
    let (instructor_id, set_instructor_id) = create_signal(0);
    let (description, set_description) = create_signal("".to_string());

    let instructors = create_resource(
        || {},
        |_| async {
            get_users_by_role("Instructor".to_string())
                .await
                .unwrap_or_default()
        },
    );

    let add_class_action = create_action(move |_| async move {
        add_class(class_name(), instructor_id(), description.get())
            .await
            .unwrap();
    });

    view! {
      <div class="p-6 bg-white rounded-lg shadow-md">
        <div class="flex justify-between items-start mb-4">
          <h2 class="mb-4 text-lg font-semibold">"New Class"</h2>
          <button
            class="py-1 px-2 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue"
            on:click=move |_| {
              display_add_class.update(|value| *value = !*value);
            }
          >
            "Close"
          </button>
        </div>
        <div class="grid grid-cols-1 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700">"Class Name"</label>
            <input
              type="text"
              class="block py-2 px-3 mt-1 w-full rounded-md border border-gray-300 shadow-sm sm:text-sm focus:border-indigo-500 focus:ring-indigo-500 focus:outline-none"
              on:input=on_input!(set_class_name)
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
          <div>
            <label class="block text-sm font-medium text-gray-700">"Class Description"</label>
            <textarea
              class="block py-2 px-3 mt-1 w-full h-24 rounded-md border border-gray-300 shadow-sm sm:text-sm focus:border-indigo-500 focus:ring-indigo-500 focus:outline-none"
              on:input=on_input!(set_description)
              prop:value=description
            />
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
fn ClassOptions(
    class: ClassInfo,
    set_display_class_options: WriteSignal<bool>,
    display_class_options: WriteSignal<bool>,
) -> impl IntoView {
    let (class_name, set_class_name) = create_signal(class.name.clone());
    let (instructor_id, set_instructor_id) = create_signal(class.instructor_id);
    let (description, set_description) = create_signal(class.description.clone());
    let (class, _set_class) = create_signal(class.clone());

    let update_class_action = create_action(move |class: &ClassInfo| {
        let class = class.clone();
        async move {
            update_class_info(class, instructor_id()).await.unwrap();
        }
    });

    let instructors = create_resource(
        || {},
        |_| async {
            get_users_by_role("Instructor".to_string())
                .await
                .unwrap_or_default()
        },
    );

    let delete_class_action = create_action(move |class: &ClassInfo| {
        let class = class.clone();
        async move {
            delete_class(class.id).await.unwrap();
            set_display_class_options(false);
            let navigate = leptos_router::use_navigate();
            navigate("/AdminHomePage", Default::default())
        }
    });

    view! {
      <div class="p-6 bg-white rounded-lg shadow-md">
        <div class="flex justify-between items-start mb-4">
          <h2 class="mb-4 text-lg font-semibold">"Class Options"</h2>
          <button
            class="py-1 px-2 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue"
            on:click=move |_| {
              display_class_options.update(|value| *value = !*value);
            }
          >
            "Close"
          </button>
        </div>
        <div class="grid grid-cols-1 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700">"Class Name"</label>
            <input
              type="text"
              class="block py-2 px-3 mt-1 w-full rounded-md border border-gray-300 shadow-sm sm:text-sm focus:border-indigo-500 focus:ring-indigo-500 focus:outline-none"
              prop:value=class.get().name
              on:input=move |ev| {
                set_class_name(event_target_value(&ev));
              }
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">"Instructor"</label>
            <select
              class="block py-2 px-3 mt-1 w-full rounded-md border border-gray-300 shadow-sm sm:text-sm focus:border-indigo-500 focus:ring-indigo-500 focus:outline-none"
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
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">"Class Description"</label>
            <textarea
              class="block py-2 px-3 mt-1 w-full h-24 rounded-md border border-gray-300 shadow-sm sm:text-sm focus:border-indigo-500 focus:ring-indigo-500 focus:outline-none"
              prop:value=description
              on:input=move |ev| {
                set_description(event_target_value(&ev));
              }
            />
          </div>
        </div>

        <div class="flex justify-between items-center mt-4">
          <button
            class="py-1 px-2 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue"
            on:click=move |_| {
              delete_class_action.dispatch(class.get());
            }
          >
            "Delete Class"
          </button>
          <button
            class="py-1 px-2 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue"
            on:click=move |_| {
              update_class_action
                .dispatch(ClassInfo {
                  id: class.get().id,
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
                  description: description.get(),
                });
            }
          >
            "Submit"
          </button>
        </div>
      </div>
    }
}
