use crate::svgs::drop_down_bars::DropDownBars;
use crate::svgs::magnifying_glass::MagnifyingGlass;
use leptos::*;

use crate::util::global_state::GlobalState;

#[component]
pub fn Header(text: String, logo: String) -> impl IntoView {
    let global_state = expect_context::<GlobalState>();
    view! {
        <div class="bg-white p-4 flex justify-between items-center text-gray-600">
            <div class="flex items-center">
                <img src={format!("/{}", logo)} alt="Logo" class="h-8 mr-2"/>
                <span class="text-xl font-bold">{text}</span>
            </div>
            <div class="relative p-2 rounded-full border border-gray-300 w-64">
                <input type="text" placeholder="Search something..." class="pl-10 pr-10 w-full"/>
                <button class="absolute inset-y-0 right-0 pr-3 flex items-center">
                    <MagnifyingGlass size="20px"/>
                </button>
            </div>
            <div class="flex items-center relative">
                <span class="text-xl font-bold mr-4 flex items-center">{global_state.user_name.get()}</span>
                <div class="flex items-center relative group">
                    <button>
                        <DropDownBars size="1.3rem"/>
                    </button>
                    <div class="absolute right-0 top-full mt-[-0.1rem] bg-white shadow-md rounded-lg transition
                        ease-out duration-200 opacity-0 scale-95 invisible group-hover:opacity-100 group-hover:scale-100
                        group-hover:visible">
                        <ul class="py-1 text-gray-700 w-36 text-left text-lg">
                            <li class="px-4 py-2 hover:bg-gray-100 cursor-pointer">
                                <div class="block">"Profile"</div>
                            </li>
                            <li class="px-4 py-2 hover:bg-gray-100 cursor-pointer">
                                <div class="block">"Settings"</div>
                            </li>
                            <li class="px-4 py-2 hover:bg-gray-100 cursor-pointer">
                                <div class="block">"Logout"</div> //Todo: Register/login?
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}
