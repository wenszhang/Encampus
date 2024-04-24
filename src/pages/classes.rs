use leptos::{component, create_resource, view, For, IntoView, Suspense};
use leptos::{create_effect, leptos_dom};

use crate::components::header::Header;
use crate::database_functions::get_class_list;
use crate::database_functions::ClassInfo;

#[component]
pub fn ClassTile(class: ClassInfo) -> impl IntoView {
    view! {
        <a href=&format!("classes/{}", class.id)>
            <div class="tile bg-white rounded shadow p-4 flex items-center justify-center font-bold h-32">
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
        <Header text="ENCAMPUS".to_string() logo={None} class_id={None} />

        <div class="grid grid-cols-3 gap-4 p-10 mx-20">
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                <For each=move || classes().unwrap_or_default() key=|class| class.id let:class>
                    <ClassTile class={class} />
                </For>
            </Suspense>
        </div>
    }
}
