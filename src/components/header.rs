use leptos::*;

#[component]
pub fn Header(text: String, logo: String) -> impl IntoView {
    view! {
        <div class="bg-white p-4 flex justify-between items-center text-gray-600">
            <div class="flex items-center">
                <img src={logo} alt="Logo" class="h-8 mr-2"/>
                <span class="text-xl font-bold">{text}</span>
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
    }
}
