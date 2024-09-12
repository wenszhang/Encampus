use crate::data::database::announcement_functions::AnnouncementInfo;
use crate::resources::images::svgs::announcement_mic::AnnouncementMic;
use leptos::*;

#[component]

//TODO handle too many announcements and href to announcements page
pub fn Announcements(announcements: Vec<AnnouncementInfo>) -> impl IntoView {
    let (is_expanded, set_is_expanded) = create_signal(true);

    let mut sorted_announcements = announcements.clone();
    sorted_announcements.sort_by(|a, b| b.time.cmp(&a.time));

    view! {
        <div class="relative rounded-lg shadow-lg flex flex-col \
                        bg-card-bg overflow-hidden">
        //Announcement header
        <div class="w-full h-7 bg-customBlue rounded-t-lg flex justify-between items-center px-3">
            <div class="flex text-white items-center">
                <AnnouncementMic size="5em" />
                <h3 class="px-2">"RECENT ANNOUNCEMENTS"</h3>
            </div>

            <div class="flex text-white items-center hover:text-customBlue-HOVER hover:bg-gray-300">
                <button on:click=move |_| set_is_expanded.update(|v| *v = !*v)>
                    <details open={is_expanded.get()}>
                        <summary>{ move || if is_expanded.get() { "COLLAPSE" } else { "EXPAND" } }</summary>
                    </details>
                </button>
            </div>
        </div>

        // Announcement info
        <div class=format!("bg-[#EEEEEE]{}", "flex flex-col")>
            { move || {
                let announcements_clone = sorted_announcements.clone();
                if is_expanded.get() {
                    view! {
                        <ul>
                            {announcements_clone.into_iter().map(|announcement| view! {
                                <li class="p-2 border-b border-gray-300 hover:bg-gray-100">
                                    <h4 class="font-bold">{announcement.title.clone()}</h4>
                                    <p class="text-sm">{announcement.contents.clone()}</p>
                                    <p class="text-xs text-gray-500">{announcement.time.format("%Y-%m-%d %H:%M:%S").to_string()}</p>
                                </li>
                            }).collect::<Vec<_>>()}
                        </ul>
                    }
                } else {
                    view! { <ul></ul> }
                }
            }}
        </div>
        </div>
    }
}
