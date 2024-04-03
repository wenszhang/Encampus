use leptos::{component, view, IntoView};

#[component]
fn ClassCard(class_name: &'static str) -> impl IntoView {
    view! {
        <div class="bg-white rounded shadow p-4 flex items-center justify-center font-bold h-32">
            {class_name}
        </div>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <div class="bg-white p-4 flex justify-between items-center text-gray-600">
            <LogoAndTitle/>
            <div class="relative">
                <input type="text" placeholder="Ask me anything..." class="p-2 rounded-full border border-gray-300 w-64"/>
                <button class="absolute right-0 top-0 mr-2 text-black">
                    <i class="fa-search"/>
                </button>
            </div>
            <div class="flex items-center">
                // TODO: Replace with vh/vw for dynamic size
                <span class="text-xl font-bold mr-4">"LONGNAME"</span>
                <div class="p-2">"☰"</div>
            </div>
        </div>
    }
}

#[component]
fn LogoAndTitle() -> impl IntoView {
    view! {
        <div class="flex items-center">
            <img src="logo.png" alt="ENCAMPUS" class="h-8 mr-2"/>
            <span class="text-xl font-bold">"ENCAMPUS"</span>
        </div>
    }
}

#[component]
pub fn TestPage() -> impl IntoView {
    view! {
        <div class="bg-gray-200 min-h-screen">
            <NavBar/>
            // TODO: Dynamically generate tiles
            <div class="grid grid-cols-3 gap-4 p-10 mx-20">
                <ClassCard class_name="Math 3210"/>
                <ClassCard class_name="Class 3124"/>
                <ClassCard class_name="Class 4123"/>
                <ClassCard class_name="Class 3214"/>
                <ClassCard class_name="Class 1243"/>
            </div>
        </div>
    }
}
