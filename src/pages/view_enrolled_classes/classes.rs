/**
* Page getting and displaying all classes registered to the user
*/
use leptos::{component, create_resource, view, For, IntoView, Signal, Suspense};
use leptos::{create_effect, leptos_dom};

use crate::data::database::class_functions::get_class_list;
use crate::data::database::class_functions::ClassInfo;
use crate::pages::global_components::header::Header;

#[component]
pub fn ClassTile(class: ClassInfo) -> impl IntoView {
    let var_name = view! {
        <a href=&format!("classes/{}", class.id)>
            <div class="relative bg-white rounded-lg shadow-lg p-6 flex flex-col items-center justify-center text-lg font-semibold h-60 w-85 transition-transform duration-300 hover:scale-105 hover:bg-gray-100 hover:shadow-xl overflow-hidden">
                <div class="flex-1 flex items-center justify-center text-center mt-2">
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
    create_effect(move |_| {
        leptos_dom::document().set_title("Encampus - Classes");
    });

    let classes = create_resource(
        || {},
        |_| async { get_class_list().await.unwrap_or_default() },
    );

    view! {
        <Header text="ENCAMPUS".to_string() logo={None} class_id={Signal::derive(|| None)} />

        <div class="grid grid-cols-3 gap-4 p-10 mx-20">
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                <For each=move || classes().unwrap_or_default() key=|class| class.id let:class>
                    <ClassTile class={class} />
                </For>
            </Suspense>
        </div>
    }
}
