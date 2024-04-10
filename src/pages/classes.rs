use leptos::{component, view, For, IntoView};

use crate::components::class_tile::ClassTile;
use crate::components::header::Header;

/**
 * Page showing all classes registered to the user
 */
#[component]
pub fn ClassesPage() -> impl IntoView {
    //TODO: Load this based on user data
    let class_ids = vec![
        "Math3210".to_string(),
        "Class3124".to_string(),
        "Class4123".to_string(),
        "Class3214".to_string(),
        "Class1243".to_string(),
    ];

    view! {
        <Header text="ENCAMPUS".to_string() logo="logo.png".to_string() />

        <div class="grid grid-cols-3 gap-4 p-10 mx-20">
            <For each=move || class_ids.clone() key=|id| id.clone() let:class_id>
                <ClassTile class_id={class_id} />
            </For>
        </div>
    }
}
