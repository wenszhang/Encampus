use crate::data::database::class_functions::get_students_classes;
use crate::data::database::class_functions::ClassInfo;
use crate::expect_logged_in_user;
use crate::pages::view_class_posts::class::ClassId;
use crate::resources::images::svgs::home_icon::HomeIcon;
use crate::resources::images::svgs::profile_icon::ProfileIcon;
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
          <ProfileIcon size="6em" />
        </div>

        <Suspense fallback=move || view! { <p>"Loading user info..."</p> }>
          <h1 class="text-2xl font-bold text-center">
            {move || user().first_name.clone()} {move || user().last_name.clone()}
          </h1>

          <h2 class="text-lg font-semibold text-center text-gray-500">
            {move || user().role.clone()}
          </h2>
        </Suspense>

          <ul>
            <li class="flex items-center py-2">
              <A
                href=move || format!("/classes/{}", class_id_val)
                class="flex gap-2 items-center py-2 px-4 text-white rounded-md hover:bg-gray-700"
              >
                <HomeIcon size="1.5em" />
                <span>"Back to Class"</span>
              </A>
            </li>
          </ul>
        </div>
    }.into_view()
}
