/**
 * Component view and logic for the header at the top of the page of the site
 */
use crate::data::database::announcement_functions::get_announcement_list;
use crate::data::global_state::GlobalState;
use crate::resources::images::svgs::announcement_bell::AnnouncementBell;
use crate::resources::images::svgs::drop_down_bars::DropDownBars;
use leptos::*;
use leptos_router::{use_navigate, A};

#[component]
pub fn AnnouncementInfo(class_id: impl Fn() -> i32 + 'static) -> impl IntoView {
    let announcements = create_resource(class_id, |class_id| async move {
        get_announcement_list(class_id).await.unwrap_or_default()
    });

    view! {
        <ul class="py-1 mx-1 text-gray-700 w-40 text-left text-lg">
        <Suspense fallback=move || view! {<li class= "px-4 py-2 hover:bg-gray-100 cursor-pointer">"Loading..."</li>}>
        {announcements()
            .map(|announcement_info_vec| announcement_info_vec.into_iter().map(|announcement_info| view! { <li class= "px-4 py-2 hover:bg-gray-100 cursor-pointer">{announcement_info.title}</li>}).collect_view())
            }
        </Suspense>
        </ul>
    }
}

#[component]
pub fn Header(text: String, logo: Option<String>, class_id: Signal<Option<i32>>) -> impl IntoView {
    let global_state = expect_context::<GlobalState>(); // Access global state
    let navigate = use_navigate(); // Create a navigation function
    let logo_src = logo.as_deref().unwrap_or("images/BlockU_RGB.png");

    // Clone global_state so it can be used in multiple closures
    let global_state_clone = global_state.clone();
    let global_state_clone_for_first_name = global_state.clone();

    let header_text_href = move || {
        if let Some(id) = class_id() {
            format!("/classes/{}", id)
        } else {
            "/classes".to_string()
        }
    };

    let logout = move |_| {
        // Clear local storage and reset global state
        global_state_clone.clear_local_storage();
        // Redirect the user to the login page after logging out
        navigate("/login", Default::default());
    };

    view! {
        <div class="bg-white p-4 flex justify-between items-center text-gray-600 ">
            <div class="flex items-center">
                <a href="/classes"><img src={format!("/{}", logo_src)} alt="Logo" class="h-8 mr-2"/></a>
                <a href={header_text_href} class="text-xl font-bold">{text}</a>
            </div>

            <div class="flex items-center">
                <Suspense fallback=move || view! {<span>"Loading announcements..."</span>}>
                    {move || class_id().map(|class_id: i32| view! {
                        <div class="group relative">
                        <span class="inline-flex items-baseline">
                            <h3 class="px-2"> "New Announcements"</h3>
                                <button class="pr-2">
                                    <AnnouncementBell size="1.3rem"/>
                                </button>
                        </span>
                            <div class="absolute right-0 top-full mt-[-0.1rem] shadow-md rounded-lg bg-white invisible
                                group-hover:opacity-100 group-hover:scale-100 group-hover:visible">
                                <AnnouncementInfo class_id = move || class_id/>
                            </div>
                        </div>
                    }).unwrap_or_else(|| view! { <div>"No announcements"</div> })}
                </Suspense>

                <span class="text-xl font-bold mr-4 flex items-center">{move || global_state_clone_for_first_name.first_name.get()}</span>
                <div class="flex items-center relative group">
                    <button>
                        <DropDownBars size="1.3rem"/>
                    </button>
                    <div class="absolute right-0 top-full mt-[-0.1rem] bg-white shadow-md rounded-lg transition
                        ease-out duration-200 opacity-0 scale-95 invisible group-hover:opacity-100 group-hover:scale-100
                        group-hover:visible z-50">
                        <ul class="py-1 text-gray-700 w-36 text-left text-lg">
                            <li class="px-4 py-2 hover:bg-gray-100 cursor-pointer">
                                <div class="block">"Profile"</div>
                            </li>
                            <li class="px-4 py-2 hover:bg-gray-100 cursor-pointer">
                                <div class="block">"Settings"</div>
                            </li>
                            <li class="px-4 py-2 hover:bg-gray-100 cursor-pointer">
                                <div class="block">
                                    <A href="/classes">"Dashboard"</A>
                                </div>
                            </li>
                            <li class="px-4 py-2 hover:bg-gray-100 cursor-pointer">
                                <div class="block" on:click=logout>
                                    "Logout"
                                </div>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}
