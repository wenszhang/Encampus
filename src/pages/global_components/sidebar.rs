use crate::data::database::class_functions::get_users_classes;
use crate::data::database::class_functions::ClassInfo;
use crate::expect_logged_in_user;
use crate::pages::view_class_posts::class::ClassId;
use leptos::*;
use leptos_router::{use_params, A};

#[component]
pub fn Sidebar() -> impl IntoView {
    let (user, _) = expect_logged_in_user!();
    let (collapsed, set_collapsed) = create_signal(false);
    let user_role = move || user().role;
    let curr_class_id = use_params::<ClassId>();
    let class_id_val = curr_class_id
        .get_untracked()
        .map(|class_id| class_id.class_id)
        .unwrap_or(0);

    let valid_class_id_val = if class_id_val != 0 {
        Some(class_id_val)
    } else {
        None
    };

    let courses = create_resource(
        || {},
        move |_| {
            let id = user().id;
            // async move { get_students_classes(id).await.unwrap_or_default() }
            let role = user_role();
            async move { get_users_classes(id, role).await.unwrap_or_default() }
        },
    );

    let collapse_class = move || {
        if collapsed.get() {
            "sticky top-0 h-screen w-8 bg-gray-800 text-white flex items-center justify-center"
        } else {
            "sticky top-0 h-screen w-64 bg-gray-800 text-white"
        }
    };

    view! {
      <div class=collapse_class>
        {move || {
          if collapsed.get() {
            collapsed_view(set_collapsed).into_view()
          } else {
            expanded_view(set_collapsed, courses, valid_class_id_val).into_view()
          }
        }}
      </div>
    }
    .into_view()
}

// Collapsed view for the sidebar
fn collapsed_view(set_collapsed: WriteSignal<bool>) -> View {
    view! {
      <button
        class="flex justify-center items-center w-full h-full text-2xl text-white"
        on:click=move |_| set_collapsed.update(|c| *c = !*c)
      >
        "⮞"
      </button>
    }
    .into_view()
}

// Expanded view for the sidebar
fn expanded_view(
    set_collapsed: WriteSignal<bool>,
    courses: Resource<(), Vec<ClassInfo>>,
    class_id_val: Option<i32>,
) -> View {
    let (user, _) = expect_logged_in_user!();

    view! {
      <div class="flex flex-col h-full">
        // Profile Image and User Info
        <div class="flex justify-center items-center mt-10 mb-4">
          <img
            src="https://static.vecteezy.com/system/resources/thumbnails/009/292/244/small/default-avatar-icon-of-social-media-user-vector.jpg"
            class="w-16 h-16 rounded-full"
            alt="Profile Image"
          />
        </div>

        // Reactive Name and Role
        <Suspense fallback=move || view! { <p>"Loading user info..."</p> }>
          <h1 class="text-2xl font-bold text-center">
            {move || user().first_name.clone()} " " {move || user().last_name.clone()}
          </h1>

          <h2 class="text-lg font-semibold text-center text-gray-500">
            {move || user().role.clone()}
          </h2>
        </Suspense>

        <div class="overflow-y-auto flex-grow px-4 mt-6 custom-scrollbar">
          <h2 class="mb-2 text-sm tracking-widest text-gray-400 uppercase">"Fall 24 Courses"</h2>

          // Wrapping the entire course list in <Suspense />
          <Suspense fallback=move || view! { <p>"Loading courses..."</p> }>
            <ul>
              <For each=move || courses().unwrap_or_default() key=|class| class.id let:class>
                <li class="py-2">
                  <A
                    href=format!("/classes/{}", class.id)
                    target="_self"
                  >
                    <p
                      class="block py-2 px-4 text-white rounded-md hover:bg-gray-700"
                      class=("bg-gray-700", move || (class_id_val == Some(class.id)))
                    >
                      {class.name}
                    </p>
                  </A>
                </li>
              </For>
            </ul>
          </Suspense>

          <h2 class="mt-6 mb-2 text-sm tracking-widest text-gray-400 uppercase">"Tools"</h2>
          <ul>
            <li class="py-2">
              {if let Some(class_id) = class_id_val {
                view! {
                  <div>
                    <A href=format!("/classes/{}/details", class_id)
                    class="block py-2 px-4 text-white rounded-md hover:bg-gray-700"
                    >"Class Details"
                    </A>
                  </div>
                }
              } else {
                view! { <div></div> }
              }}
            </li>
          </ul>

          <ul>
            <li class="py-2">
              {class_id_val.map(|class_id|
                view! {
                  <A href=format!("/class/{class_id}/poll")
                  class="block py-2 px-4 text-white rounded-md hover:bg-gray-700"
                  >"Live Polling"
                  </A>
                })
              }
            </li>
          </ul>
        </div>

        // Account Settings Button
        <div class="py-2 px-2 w-full bg-gray-700 rounded-md hover:bg-gray-600">
          <A href="/classes" class="block py-1 w-full text-sm text-center text-white">
            "Back To Classes"
          </A>
        </div>

        // Collapse Button
        <button
          class="absolute top-4 right-4 text-white"
          on:click=move |_| set_collapsed.update(|c| *c = !*c)
        >
          "✕"
        </button>
      </div>
    }
    .into_view()
}
