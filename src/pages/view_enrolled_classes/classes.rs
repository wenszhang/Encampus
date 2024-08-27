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
    view! {
        <a href=&format!("classes/{}", class.id)>
                <div class="tile bg-white rounded shadow-md p-10 flex items-center justify-center text-l font-bold h-70 w-50 tile transition duration-300 hover:bg-gray-100">
                    {class.name}
                </div>

        </a>
    }
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
