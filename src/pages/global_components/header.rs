use crate::data::database::announcement_functions::get_announcement_list;
/**
 * Component view and logic for the header at the top of the page of the site
 */
use crate::resources::images::svgs::announcement_bell::AnnouncementBell;
use crate::resources::images::svgs::drop_down_bars::DropDownBars;
use crate::resources::images::svgs::magnifying_glass::MagnifyingGlass;
// use crate::components::announcements::Announcements;
use leptos::*;
use svg::view;

use crate::data::global_state::GlobalState;

// database function for announcements
use crate::data::database::announcement_functions::GetAnnouncementsList;

#[component]
pub fn AnnouncementInfo(class_id: Option<i32>) -> impl IntoView {
    let announcements = create_resource(
        || class_id,
        |class_id| async {
            get_announcement_list(class_id.unwrap())
                .await
                .unwrap_or_default()
        },
    );

    return view! {
        <ul class="py-1 mx-1 text-gray-700 w-40 text-left text-lg">
        {announcements.into_iter()
            .map(|n| view! { <li class= "px-4 py-2 hover:bg-gray-100 cursor-pointer">{n}</li>})
            .collect_view()}
        </ul>
    };
}

#[component]
pub fn Header(text: String, logo: Option<String>, class_id: Option<i32>) -> impl IntoView {
    let global_state = expect_context::<GlobalState>();
    let logo_src = logo.as_deref().unwrap_or("images/logo.png");
    let header_text_href = if let Some(id) = &class_id {
        format!("/classes/{}", id)
    } else {
        "/classes".to_string()
    };
    view! {
        <div class="bg-white p-4 flex justify-between items-center text-gray-600 ">
            <div class="flex items-center">
                <a href="/classes"><img src={format!("/{}", logo_src)} alt="Logo" class="h-8 mr-2"/></a>
                <a href={header_text_href} class="text-xl font-bold">{text}</a>
            </div>
            <div class="relative p-2 rounded-full border border-gray-300 focus-within:border-blue-500 w-64">
                <input type="text" placeholder="Search something..." class="pl-5 pr-5 w-full border-none focus:outline-none"/>
                <button class="absolute inset-y-0 right-0 pr-3 flex items-center">
                    <MagnifyingGlass size="20px"/>
                </button>
            </div>
            <div class="flex items-center ">
                <div class="group relative">
                    <button class="pr-2">
                        <AnnouncementBell size="1.3rem"/>
                    </button>
                    <div class="absolute right-0 top-full mt-[-0.1rem] shadow-md rounded-lg bg-white invisible
                        group-hover:opacity-100 group-hover:scale-100 group-hover:visible">
                        <AnnouncementInfo class_id = class_id/>
                    </div>
                </div>
                <span class="text-xl font-bold mr-4 flex items-center">{move || global_state.user_name.get()}</span>
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
                                <div class="block">
                                    <a href="/classes">"Dashboard"</a>
                                </div>
                            </li>
                            <li class="px-4 py-2 hover:bg-gray-100 cursor-pointer">
                                <div class="block">
                                    <a href="/login">"Logout"</a>
                                </div>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}
