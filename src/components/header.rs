use leptos::*;
use crate::svgs::drop_down_bars::DropDownBars;
use crate::svgs::magnifying_glass::MagnifyingGlass;

#[component]
pub fn Header(text: String, logo: String) -> impl IntoView {
    view! {
        <div class="bg-white p-4 flex justify-between items-center text-gray-600">
            <div class="flex items-center">
                <img src={logo} alt="Logo" class="h-8 mr-2"/>
                <span class="text-xl font-bold">{text}</span>
            </div>
            <div class="relative p-2 rounded-full border border-gray-300 w-64">
                <input type="text" placeholder="Ask me anything..."/>
                // TODO clean up styling
                <button class="absolute mt-1 mr-3 text-black">
                    <MagnifyingGlass size="20px"/>
                </button>
            </div>
            <div class="flex items-center">
                <span class="text-xl font-bold mr-4">"LONGNAME"</span> // Todo: Replace with vh/vw for dynamic size
                <DropDownBars size="20px"/>
            </div>
        </div>
    }
}
