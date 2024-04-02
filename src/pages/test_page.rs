use leptos::{component, view, IntoView};
use leptos_meta::Stylesheet;

#[component]
pub fn TestPage() -> impl IntoView {
    
    view!{
        <div class="bg-gray-200 min-h-screen">
            <Stylesheet id="leptos" href="/pkg/encampus.css"/>
            // ToDo: Cleanup, extract Tailwind to reduce styling in code
            <div class="bg-white p-4 flex justify-between items-center text-gray-600">
                <div class="flex items-center">
                    <img src="logo.png" alt="ENCAMPUS" class="h-8 mr-2"/>
                    <span class="text-xl font-bold">"ENCAMPUS"</span>
                </div>
                <div class="relative">
                    <input type="text" placeholder="Ask me anything..." class="p-2 rounded-full border border-gray-300 w-64"/>
                    <button class="absolute right-0 top-0 mr-2 text-black">
                        <i class="fas fa-search"/>
                    </button>
                </div>
                <div class="flex items-center">
                    <span class="text-xl font-bold mr-4">"LONGNAME"</span> // Todo: Replace with vh/vw for dynamic size
                    <div class="p-2">"☰"</div>
                </div>
            </div>
            // ToDo: Dynamically generate tiles
            <div class="grid grid-cols-3 gap-4 p-10 mx-20">
            <div class="tile bg-white rounded shadow p-4 flex items-center justify-center font-bold h-32">
                "Math 3210"
            </div>
            <div class="tile bg-white rounded shadow p-4 flex items-center justify-center font-bold h-32">
                "Class 3124"
            </div>
            <div class="tile bg-white rounded shadow p-4 flex items-center justify-center font-bold h-32">
                "Class 4123"
            </div>
            <div class="tile bg-white rounded shadow p-4 flex items-center justify-center font-bold h-32">
                "Class 3214"
            </div>
            <div class="tile bg-white rounded shadow p-4 flex items-center justify-center font-bold h-32">
                "Class 1243"
            </div>
        </div>
        </div>
    }
}
