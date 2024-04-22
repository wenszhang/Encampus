use leptos::leptos_dom;
use leptos::{component, create_resource, view, For, IntoView, Suspense};

use crate::components::header::Header;
use crate::database_functions::get_class_list;
use crate::database_functions::ClassInfo;

#[component]
pub fn ClassTile(class: ClassInfo) -> impl IntoView {
    view! {
        <a href=&format!("class/{}", class.id)>
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
    leptos_dom::document().set_title("Encpamus - Classes");
    let classes = create_resource(|| {}, |_| async { get_class_list().await.unwrap() });

    view! {
        <Header text="ENCAMPUS".to_string() logo="logo.png".to_string() />

        <div class="grid grid-cols-3 gap-4 p-10 mx-20">
            <Suspense
                fallback=move || view! { <p>"Loading..."</p> }
            >
                <For each=move || classes().unwrap_or_default() key=|class| class.id let:class>
                    <ClassTile class={class} />
                </For>

                // {classes_list}
            </Suspense>
        </div>
    }
}
