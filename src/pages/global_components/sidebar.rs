use crate::data::database::class_functions::get_users_classes;
use crate::expect_logged_in_user;
use crate::pages::view_class_posts::class::ClassId;
use leptos::*;
use leptos_router::{use_params, A};

#[component]
pub fn Sidebar() -> impl IntoView {
  let (collapsed, set_collapsed) = create_signal(false);

  let collapse_class = move || {
      if collapsed.get() {
          "sticky top-0 h-screen w-8 bg-gray-800 text-white flex items-center justify-center cursor-pointer"
      } else {
          "sticky top-0 h-screen w-64 bg-gray-800 text-white"
      }
  };

    view! {
      <div
        class=collapse_class
        on:click=move |_| {
          if collapsed.get() {
            set_collapsed.set(false)
          }
        }
      >
        {move || {
          if collapsed.get() {
            view! { <CollapsedView /> }
          } else {
            view! { <ExpandedView handle_collapse_button=move || set_collapsed.set(true) /> }
          }
        }}
      </div>
    }
}

// Collapsed view for the sidebar
#[component]
fn CollapsedView() -> impl IntoView {
    view! {
      <div class="flex justify-center items-center w-full h-full font-bold text-white whitespace-nowrap -rotate-90">
        "OPEN NAVIGATION ▼"
      </div>
    }
}

// Expanded view for the sidebar
#[component]
fn ExpandedView(
  handle_collapse_button: impl Fn() + 'static + Copy
) -> impl IntoView {
    let (user, _) = expect_logged_in_user!();

    let class_id = {
        let class_params = use_params::<ClassId>();
        move || class_params().ok().map(|params| params.class_id)
    };

    let courses = create_resource(
        user,
        move |user| {
            let id = user.id;
            let role = user.role;
            async move { get_users_classes(id, role).await.unwrap_or_default() }
        },
    );

    view! {
      <div class="flex flex-col h-full">
        // Profile Image and User Info
        <div class="flex justify-center items-center mt-10 mb-4">
          <img
            src="/images/user_profile/UserProfileIcon.svg"
            alt="User Profile Icon"
            class="w-24 h-24 rounded-full"
          />
        </div>

        // Reactive Name and Role
        <h1 class="text-2xl font-bold text-center">
          {move || user().first_name.clone()} " " {move || user().last_name.clone()}
        </h1>

        <h2 class="text-lg font-semibold text-center text-gray-500">
          {move || user().role.clone()}
        </h2>

        <div class="overflow-y-auto flex-grow px-4 mt-6 custom-scrollbar">
          <h2 class="mb-2 text-sm tracking-widest text-gray-400 uppercase">"Fall 24 Courses"</h2>

          // Wrapping the entire course list in <Suspense />
          <Suspense fallback=move || view! { <p>"Loading courses..."</p> }>
            <ul>
              <For each=move || courses().unwrap_or_default() key=|class| class.id let:class>
                <li class="py-2">
                  <A href=format!("/classes/{}", class.id)>
                    <p
                      class="block py-2 px-4 text-white rounded-md hover:bg-gray-700"
                      class=("bg-gray-700", move || class_id() == Some(class.id))
                    >
                      {class.name}
                    </p>
                  </A>
                </li>
              </For>
            </ul>
          </Suspense>

          {move || class_id().map(|class_id| 
            view! {
              <div>
                <h2 class="mt-6 mb-2 text-sm tracking-widest text-gray-400 uppercase">"Tools"</h2>
                <ul>
                  <li class="py-2">
                    <div>
                      <A
                        href=move || format!("/classes/{}/details", class_id)
                        class="block py-2 px-4 text-white rounded-md hover:bg-gray-700"
                      >
                        "Class Details"
                      </A>
                    </div>
                  </li>
                </ul>

                <ul>
                  <li class="py-2">
                    <A
                      href=move || format!("/class/{}/poll", class_id)
                      class="block py-2 px-4 text-white rounded-md hover:bg-gray-700"
                    >
                      "Live Polling"
                    </A>
                  </li>
                </ul>
              </div>
            })
          }
        </div>

        // Back to classes button.
        <div class="py-2 px-2 w-full bg-gray-700 rounded-md hover:bg-gray-600">
          <A href="/classes" class="block py-1 w-full text-sm text-center text-white">
            "Back To Classes"
          </A>
        </div>

        // Collapse Button
        <button
          class="absolute top-4 right-4 text-white"
          on:click=move |_| handle_collapse_button()
        >
          "✕"
        </button>
      </div>
    }
    .into_view()
}
