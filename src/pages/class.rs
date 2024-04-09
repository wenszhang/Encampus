use leptos::{component, view, IntoView};
use leptos_meta::Stylesheet;

use crate::components::class_tile::ClassTile;
use crate::components::header::Header;

#[component]
pub fn ClassPage() -> impl IntoView {
    view! {
        <div class="bg-gray-200 min-h-screen">
            <Stylesheet id="leptos" href="/pkg/encampus.css"/>

            <Header text="ENCAMPUS".to_string() logo="logo.png".to_string() />

            <div class="grid grid-cols-3 gap-4 p-10 mx-20">
                <ClassTile title="Math 3210".to_string() />
                <ClassTile title="Class 3124".to_string() />
                <ClassTile title="Class 4123".to_string() />
                <ClassTile title="Class 3214".to_string() />
                <ClassTile title="Class 1243".to_string() />
            </div>
        </div>
    }
}
