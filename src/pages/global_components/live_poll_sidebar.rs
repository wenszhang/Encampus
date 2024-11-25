use crate::data::database::class_functions::get_students_classes;
use crate::data::database::class_functions::ClassInfo;
use crate::expect_logged_in_user;
use crate::pages::view_class_posts::class::ClassId;
use crate::resources::images::svgs::home_icon::HomeIcon;
use leptos::*;
use leptos_router::{use_params, A};

#[component]
pub fn Sidebar() -> impl IntoView {
    let (user, _) = expect_logged_in_user!();
    let (collapsed, set_collapsed) = create_signal(false);
    let curr_class_id = use_params::<ClassId>();
    let class_id_val = curr_class_id.get_untracked().unwrap().class_id;

    let courses = create_resource(
        || {},
        move |_| {
            let id = user().id;
            async move { get_students_classes(id).await.unwrap_or_default() }
        },
    );

    let class = if collapsed.get() {
        "sticky top-0 h-screen w-8 bg-gray-800 text-white flex items-center justify-center"
    } else {
        "sticky top-0 h-screen w-64 bg-gray-800 text-white"
    };

    // Collapsed view for the sidebar
    fn collapsed_view(set_collapsed: WriteSignal<bool>) -> View {
        view! {
          <button class="text-2xl text-white" on:click=move |_| set_collapsed.update(|c| *c = !*c)>
            "→"
          </button>
        }
        .into_view()
    }

    view! {
      <div class=class>
        {if collapsed.get() {
          collapsed_view(set_collapsed).into_view()
        } else {
          expanded_view(set_collapsed, courses, class_id_val).into_view()
        }}
      </div>
    }
    .into_view()
}

// Expanded view for the sidebar
fn expanded_view(
    set_collapsed: WriteSignal<bool>,
    courses: Resource<(), Vec<ClassInfo>>,
    class_id_val: i32,
) -> View {
    let (user, _) = expect_logged_in_user!();
    view! {
      <div class="flex flex-col h-full">
        <div class="flex justify-center items-center mt-10 mb-4">
          <img
            src="https://static.vecteezy.com/system/resources/thumbnails/009/292/244/small/default-avatar-icon-of-social-media-user-vector.jpg"
            class="w-16 h-16 rounded-full"
            alt="Profile Image"
          />
        </div>

        <Suspense fallback=move || view! { <p>"Loading user info..."</p> }>
          <h1 class="text-2xl font-bold text-center">
            {move || user().first_name.clone()} {move || user().last_name.clone()}
          </h1>

          <h2 class="text-lg font-semibold text-center text-gray-500">
            {move || user().role.clone()}
          </h2>
        </Suspense>

        <div class="overflow-y-auto flex-grow px-4 mt-6 custom-scrollbar">
          // EDIT THIS TO DISPLAY WHATEVER COURSE YOU ARE IN
          <h2 class="mb-2 text-sm tracking-widest text-gray-400 uppercase">"Fall 24 Courses"</h2>
          <Suspense fallback=move || view! { <p>"Loading courses..."</p> }>
            <ul>
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
            </ul>
          </Suspense>

          <ul>
            <li class="flex items-center py-2">
              <A
                href=move || format!("/classes/{}", class_id_val)
                class="flex gap-2 items-center py-2 px-4 text-white rounded-md hover:bg-gray-700"
              >
                <HomeIcon size="1em" />
                <span>"Back to Class Page"</span>
              </A>
            </li>
          </ul>
        </div>
      </div>
    }.into_view()
}
