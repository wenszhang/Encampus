use crate::data::database::class_functions::get_students_classes;
use crate::data::database::class_functions::ClassInfo;
use crate::data::global_state::GlobalState;
use leptos::*;
use leptos_router::A;

#[component]
pub fn Sidebar() -> impl IntoView {
    let global_state = expect_context::<GlobalState>(); // Access global state
    let (collapsed, set_collapsed) = create_signal(false);

    let courses = create_resource(
        || {},
        move |_| {
            let id = global_state.id.get().unwrap_or_default();
            async move { get_students_classes(id).await.unwrap_or_default() }
        },
    );

    let class = if collapsed.get() {
        "sticky top-0 h-screen w-8 bg-gray-800 text-white flex items-center justify-center"
    } else {
        "sticky top-0 h-screen w-64 bg-gray-800 text-white"
    };

    view! {
      <div class=class>
        {if collapsed.get() {
          collapsed_view(set_collapsed).into_view()
        } else {
          expanded_view(set_collapsed, courses, global_state).into_view()
        }}
      </div>
    }
}

// Collapsed view for the sidebar
fn collapsed_view(set_collapsed: WriteSignal<bool>) -> View {
    view! {
      <button class="text-2xl text-white" on:click=move |_| set_collapsed.update(|c| *c = !*c)>
        "→"
      </button>
    }
    .into_view()
}

// Expanded view for the sidebar
fn expanded_view(
    set_collapsed: WriteSignal<bool>,
    courses: Resource<(), Vec<ClassInfo>>,
    global_state: GlobalState,
) -> View {
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
        <h1 class="text-2xl font-bold text-center">
          {move || {
            let first_name = global_state.first_name.get();
            let last_name = global_state.last_name.get();
            format!("{} {}", first_name.unwrap_or_default(), last_name.unwrap_or_default())
          }}
        </h1>
        <h2 class="text-lg font-semibold text-center text-gray-500">
          {move || global_state.role.get().unwrap_or_default()}
        </h2>

        <div class="overflow-y-auto flex-grow px-4 mt-6 custom-scrollbar">
          <h2 class="mb-2 text-sm tracking-widest text-gray-400 uppercase">"Fall 24 Courses"</h2>
          <ul>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
              <For each=move || courses().unwrap_or_default() key=|class| class.id let:class>
                <li class="py-2">
                  <A
                    href=format!("/classes/{}", class.id)
                    target="_self"
                    class="block py-2 px-4 text-white rounded-md hover:bg-gray-700"
                  >
                    {class.name}
                  </A>
                </li>
              </For>
            </Suspense>
          </ul>

        // <h2 class="mt-6 mb-2 text-sm tracking-widest text-gray-400 uppercase">"Tools"</h2>
        // <ul>
        // <li class="py-2">
        // <A href="/classes" class="block py-2 px-4 text-white rounded-md hover:bg-gray-700">
        // "Private Messages"
        // </A>
        // </li>
        // <li class="py-2">
        // <A href="/classes" class="block py-2 px-4 text-white rounded-md hover:bg-gray-700">
        // "Course Statistics"
        // </A>
        // </li>
        // <li class="py-2">
        // <A href="/classes" class="block py-2 px-4 text-white rounded-md hover:bg-gray-700">
        // "PollV"
        // </A>
        // </li>
        // </ul>
        </div>

        // Account Settings Button
        <div class="py-2 px-2 w-full bg-gray-700 rounded-md hover:bg-gray-600">
          <A href="/settings" class="block py-1 w-full text-sm text-center text-white">
            "Account Settings"
          </A>
        </div>

        // Collapse Button
        <button
          class="absolute top-4 right-4 text-white"
          on:click=move |_| set_collapsed.update(|c| *c = !*c)
        >
          "←"
        </button>
      </div>
    }
    .into_view()
}
