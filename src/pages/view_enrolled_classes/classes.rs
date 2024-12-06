/**
* Page getting and displaying all classes registered to the user
*/
use leptos::{component, create_resource, view, For, IntoView, Suspense};
use leptos::{create_effect, leptos_dom};
use crate::data::database::class_functions::{get_users_classes, ClassInfo};
use crate::expect_logged_in_user;
use crate::pages::global_components::header::Header;
use crate::resources::tile_design::TileDesign;

#[component]
pub fn ClassTile(class: ClassInfo) -> impl IntoView {
    let design = match class.id % 4 {
        0 => TileDesign::CircleCrayonStroke,
        1 => TileDesign::ThickCrayonStroke,
        2 => TileDesign::SquiggleCrayonStroke,
        _ => TileDesign::SwirlCrayonStroke,
    };
    let var_name = view! {
      <a href=&format!("classes/{}", class.id)>
      <div class={format!("flex overflow-hidden relative flex-col justify-center items-center p-6 h-60 text-lg font-semibold {} rounded-lg shadow-lg transition-transform duration-300 {} hover:shadow-xl hover:scale-105 w-85",
        design.get_bg_color(),
        design.get_hover_color())}>
          <div class="absolute h-600 w-600">
            <img src=design.get_svg_path()
            alt="Class Tile Design"
             />
          </div>
          <div class="absolute inset-0 bg-black/15"></div>
          <div class="flex flex-1 justify-center items-center mt-2 text-center text-white relative z-10">
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
      <Header />
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
