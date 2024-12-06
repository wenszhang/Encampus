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
    let class_id = {
      let class_params = use_params::<ClassId>();
      move || class_params().expect("Tried to render class page without class id").class_id
    };

    let courses = create_resource(
        || {},
        move |_| {
            let id = user().id;
            async move { get_students_classes(id).await.unwrap_or_default() }
        },
    );

    view! {
      <div class="sticky top-0 h-screen w-64 bg-gray-800 text-white">
        {move || view! {<ExpandedView courses class_id_val=class_id() />}}
      </div>
    }
    .into_view()
}

// Expanded view for the sidebar
#[component]
fn ExpandedView(courses: Resource<(), Vec<ClassInfo>>, class_id_val: i32) -> impl IntoView {
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
