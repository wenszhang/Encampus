/**
* Page getting and displaying all classes registered to the user
*/
use leptos::{component, create_resource, view, For, IntoView, Signal, Suspense};
use leptos::{create_effect, leptos_dom};

use crate::data::database::class_functions::{get_users_classes, ClassInfo};
use crate::expect_logged_in_user;
use crate::pages::global_components::header::Header;

#[component]
pub fn ClassTile(class: ClassInfo) -> impl IntoView {
    let var_name = view! {
      <a href=&format!("classes/{}", class.id)>
        <div class="flex overflow-hidden relative flex-col justify-center items-center p-6 h-60 text-lg font-semibold bg-white rounded-lg shadow-lg transition-transform duration-300 hover:bg-gray-100 hover:shadow-xl hover:scale-105 w-85">
          <div class="flex flex-1 justify-center items-center mt-2 text-center">
            <span>{class.name}</span>
          </div>
        </div>
      </a>
    };
    var_name
}

/**
 * Page showing all classes registered to the user
 */
#[component]
pub fn ClassesPage() -> impl IntoView {
    let (user, _) = expect_logged_in_user!();
    create_effect(move |_| {
        leptos_dom::document().set_title("Encampus - Classes");
    });

    let classes = create_resource(
        || {},
        move |_| {
            let id = user().id;
            let role = user().role;
            async move { get_users_classes(id, role).await.unwrap_or_default() }
        },
    );

    view! {
      <Header text="ENCAMPUS".to_string() logo=None class_id=Signal::derive(|| None) />

      <div class="px-10 mt-10">
        <h1 class="text-3xl font-bold leading-tight text-gray-900">Your Courses</h1>
      </div>

      <div class="grid grid-cols-3 gap-4 p-10 mx-20">
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
          <For each=move || classes().unwrap_or_default() key=|class| class.id let:class>
            <ClassTile class=class />
          </For>
        </Suspense>
      </div>
    }
    .into_view()
}
