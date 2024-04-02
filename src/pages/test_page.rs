use leptos::{component, view, IntoView};
use leptos_meta::Stylesheet;

#[component]
pub fn TestPage() -> impl IntoView {
    
    view!{
        <Stylesheet id="leptos" href="/pkg/encampus.css"/>

        <div class="bg-red-600 p-4 flex justify-between items-center text-white">
            <div class="flex items-center">
                <img src="logo.png" alt="ENCAMPUS" class="h-8 mr-2"/>
                <span class="text-lg font-semibold">"ENCAMPUS"</span>
            </div>
            <div class="relative">
                <input type="text" placeholder="Ask me anything..." class="p-2 rounded"/>
                <button class="absolute right-0 top-0 mr-2 text-black">
                    <i class="fas fa-search"/>
                </button>
            </div>
            <div class="flex items-center">
                <span class="mr-4">"LONGNAME"</span>
                <div class="p-2">"☰"</div>
            </div>
        </div>
        // TODO: Dynamically generate tiles
        <div class="flex flex-wrap justify-center gap-4 p-4">
            <div class="tile bg-white rounded shadow p-4 w-32 h-24 flex items-center justify-center font-bold">
                "Math 3210"
            </div>
            <div class="tile bg-white rounded shadow p-4 w-32 h-24 flex items-center justify-center font-bold">
                "Class 3124"
            </div>
            <div class="tile bg-white rounded shadow p-4 w-32 h-24 flex items-center justify-center font-bold">
                "Class 4123"
            </div>
            <div class="tile bg-white rounded shadow p-4 w-32 h-24 flex items-center justify-center font-bold">
                "Class 3214"
            </div>
            <div class="tile bg-white rounded shadow p-4 w-32 h-24 flex items-center justify-center font-bold">
                "Class 1243"
            </div>
            <div class="tile bg-white rounded shadow p-4 w-32 h-24 flex items-center justify-center font-bold">
                "Class 2341"
            </div>
        </div>
    }
}
