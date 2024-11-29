use crate::data::database::announcement_functions::{
    delete_announcement, post_announcement, AddAnnouncementInfo, AnnouncementInfo,
};
use crate::data::database::class_functions::check_user_is_instructor;
use crate::expect_logged_in_user;
use crate::pages::view_class_posts::class::ClassId;
use crate::resources::images::svgs::announcement_mic::AnnouncementMic;
use crate::resources::images::svgs::announcement_mic_2::AnnouncementMicAlt;

use crate::resources::images::svgs::dots_icon::DotsIcon;
use crate::resources::images::svgs::remove_icon::RemoveIcon;
use leptos::*;
use leptos_router::{use_params, A};

#[component]
pub fn AddAnnouncementModal(
    show: ReadSignal<bool>,
    set_show: WriteSignal<bool>,
    title: ReadSignal<String>,
    set_title: WriteSignal<String>,
    contents: ReadSignal<String>,
    set_contents: WriteSignal<String>,
    on_submit: Action<AddAnnouncementInfo, ()>,
    class_id: i32,
) -> impl IntoView {
    view! {
        {move || if show.get() {
            view! {
                <div class="fixed inset-0 z-50 overflow-auto bg-black/50 flex items-center justify-center">
                    <div class="relative bg-white rounded-lg shadow-xl max-w-2xl w-full m-4 p-6">
                        // Close button
                        <button
                            class="absolute top-4 right-4 text-gray-400 hover:text-gray-600"
                            on:click=move |_| set_show.set(false)
                        >
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                            </svg>
                        </button>

                        // Create announcement pop up
                        <div class="mt-4">
                            <h2 class="text-2xl font-bold mb-6">"Add New Announcement"</h2>
                            <div class="flex flex-col gap-4">
                                <div>
                                    <label class="block text-sm font-medium text-gray-700 mb-1">"Title"</label>
                                    <input
                                        class="w-full p-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-customBlue focus:border-transparent"
                                        type="text"
                                        placeholder="Announcement Title"
                                        prop:value=title
                                        on:input=move |ev| set_title.set(event_target_value(&ev))
                                    />
                                </div>
                                <div>
                                    <label class="block text-sm font-medium text-gray-700 mb-1">"Contents"</label>
                                    <textarea
                                        class="w-full p-2 border border-gray-300 rounded-md h-32 focus:ring-2 focus:ring-customBlue focus:border-transparent"
                                        placeholder="Announcement Contents"
                                        prop:value=contents
                                        on:input=move |ev| set_contents.set(event_target_value(&ev))
                                    />
                                </div>
                                <div class="flex justify-end gap-3 mt-4">
                                    <button
                                        class="px-4 py-2 text-gray-600 rounded-full hover:bg-gray-100"
                                        on:click=move |_| set_show.set(false)
                                    >
                                        "Cancel"
                                    </button>
                                    <button
                                        class="px-4 py-2 text-white rounded-full bg-customBlue hover:bg-customBlue-HOVER"
                                        on:click=move |_| {
                                            let new_announcement = AddAnnouncementInfo {
                                                title: title.get(),
                                                contents: contents.get(),
                                                class_id,
                                            };
                                            on_submit.dispatch(new_announcement);
                                            set_show.set(false);
                                            set_title.set(String::new());
                                            set_contents.set(String::new());
                                        }
                                    >
                                        "Post Announcement +"
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            }
        } else {
            view! { <div></div> }
        }}
    }
}

#[component]
pub fn AnnouncementDropDownMenu(
    announcement_id: i32,
    announcement_author_id: i32,
    delete_action: Action<i32, ()>,
) -> impl IntoView {
    let (user, _) = expect_logged_in_user!();
    let is_author = move || user().id == announcement_author_id;

    view! {
        <div class="py-1">
            {move || {
                if is_author() {
                    view! {
                        <div class="p-1">
                            <button
                                class="inline-flex items-center p-2 w-full text-sm leading-tight text-red-500 rounded-md hover:text-red-500 hover:bg-gray-100"
                                on:click=move |_| delete_action.dispatch(announcement_id)
                            >
                                <RemoveIcon size="20px" />
                                <span class="ml-2">Remove</span>
                            </button>
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }
            }}
        </div>
    }.into_view()
}

#[component]
pub fn Announcements(announcements: Vec<AnnouncementInfo>) -> impl IntoView {
    let (user, _) = expect_logged_in_user!();
    let class_id = use_params::<ClassId>();
    let class_id_val = class_id.get_untracked().unwrap().class_id;

    let is_instructor = create_resource(class_id, move |class_id| {
        let user_id = user().id;
        async move {
            check_user_is_instructor(user_id, class_id.unwrap().class_id)
                .await
                .unwrap_or(false)
        }
    });

    let delete_announcement_action = create_action(move |announcement_id: &i32| {
        let announcement_id = *announcement_id;
        async move {
            match delete_announcement(announcement_id).await {
                Ok(_) => {}
                Err(_) => logging::error!("Failed to delete announcement"),
            }
        }
    });

    let (is_adding_post, set_is_adding_post) = create_signal(false);
    let (is_expanded, set_is_expanded) = create_signal(true);
    let (title, set_title) = create_signal(String::new());
    let (contents, set_contents) = create_signal(String::new());
    let (viewing_announcement, set_viewing_announcement) = create_signal(false);
    let (selected_announcement, set_selected_announcement) =
        create_signal(None::<AnnouncementInfo>);

    let mut sorted_announcements = announcements.clone();
    sorted_announcements.sort_by(|a, b| b.time.cmp(&a.time));

    let add_announcement_action = create_action(move |announcement_info: &AddAnnouncementInfo| {
        let announcement_info = announcement_info.clone();
        async move {
            match post_announcement(announcement_info, user().id).await {
                Ok(_) => {}
                Err(_) => logging::error!("Failed to post announcement. Please try again"),
            }
        }
    });

    view! {
        <div class="flex overflow-hidden relative flex-col w-full rounded-lg shadow-lg bg-white">
            <div class="flex flex-col w-full bg-customBlue rounded-t-lg">
                <div class="flex justify-between items-center px-3 h-7">
                    <div class="flex items-center text-white">
                        <AnnouncementMic size="5em" />
                        <h3 class="px-2">"RECENT ANNOUNCEMENTS"</h3>
                    </div>

                    <div class="flex items-center text-white hover:text-gray-400">
                        <button on:click=move |_| set_is_expanded.update(|v| *v = !*v)>
                            <details open=is_expanded.get()>
                                <summary>{move || if is_expanded.get() { "COLLAPSE" } else { "EXPAND" }}</summary>
                            </details>
                        </button>
                    </div>
                </div>
            </div>

            {move || {
                let announcements_clone = sorted_announcements.clone();
                if is_expanded.get() {
                    view! {
                        <>
                            <Suspense fallback=move || view! { <p>"Loading ..."</p> }>
                                {move || {
                                    if is_instructor().unwrap_or_default() {
                                        view! {
                                            <div class="flex justify-end px-4 pt-2">
                                                <button
                                                    class="px-3 py-1 text-white rounded-full bg-customBlue hover:bg-customBlue-HOVER"
                                                    on:click=move |_| set_is_adding_post.set(true)
                                                >
                                                    "Add New Announcement +"
                                                </button>
                                            </div>
                                        }.into_view()
                                    } else {
                                        view! { <div></div> }.into_view()
                                    }
                                }}
                            </Suspense>
                            <div class="h-[290px] overflow-y-auto">
                                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 p-4 bg-white">
                                    {announcements_clone
                                        .into_iter()
                                        .map(|announcement| {
                                            let announcement_clone = announcement.clone();
                                            let (menu_visible, set_menu_visible) = create_signal(false);
                                            let is_author = user().id == announcement.author_id;

                                            view! {
                                                <div class="relative">
                                                    <div
                                                        class="cursor-pointer"
                                                        on:click=move |_| {
                                                            set_selected_announcement.set(Some(announcement_clone.clone()));
                                                            set_viewing_announcement.set(true);
                                                        }
                                                    >
                                                        <div class="flex overflow-hidden relative flex-col justify-between p-6 h-60 bg-card-header rounded-lg shadow-lg transition-transform duration-300 hover:bg-gray-100 hover:shadow-xl hover:scale-105">
                                                        //<div class="flex overflow-hidden relative flex-col justify-between p-6 h-60 bg-blue-50 rounded-lg shadow-sm transition-all duration-300 hover:bg-blue-100 hover:shadow-md hover:scale-105">
                                                            {move || if is_author {
                                                                view! {
                                                                    <div class="flex absolute top-1 right-2 z-20 items-center">
                                                                        <button
                                                                           class="rounded-lg bg-card-header hover:shadow-customInset"
                                                                            // class="rounded-lg bg-blue-50 hover:shadow-customInset"
                                                                            on:click=move |e| {
                                                                                e.stop_propagation();
                                                                                set_menu_visible.update(|value| *value = !*value);
                                                                              }
                                                                        >
                                                                            <DotsIcon size="36px"/>
                                                                        </button>
                                                                        {move || if menu_visible.get() {
                                                                            view! {
                                                                                <div class="absolute right-0 top-0 mt-7 w-30 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5">
                                                                                    <AnnouncementDropDownMenu
                                                                                        announcement_id=announcement.announcement_id
                                                                                        announcement_author_id=announcement.author_id
                                                                                        delete_action=delete_announcement_action
                                                                                    />
                                                                                </div>
                                                                            }.into_view()
                                                                        } else {
                                                                            view! { <div></div> }.into_view()
                                                                        }}
                                                                    </div>
                                                                }.into_view()
                                                            } else {
                                                                view! { <div></div> }.into_view()
                                                            }}

                                                            <div class="flex-1">
                                                                <div class="flex items-center mb-3">
                                                                <h4 class="text-lg mt-2 font-semibold  text-customBlue">{announcement.title.clone()}</h4>
                                                            </div>
                                                                <p class="text-sm text-gray-600 line-clamp-3">{announcement.contents.clone()}</p>
                                                            </div>
                                                            <p class="text-xs text-gray-500 mt-2 pl-2">
                                                                {announcement.time.format("%Y-%m-%d %H:%M:%S").to_string()}
                                                            </p>
                                                        </div>
                                                    </div>
                                                </div>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </div>
                            </div>
                        </>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }
            }}

            <AddAnnouncementModal
                show=is_adding_post
                set_show=set_is_adding_post
                title=title
                set_title=set_title
                contents=contents
                set_contents=set_contents
                on_submit=add_announcement_action
                class_id=class_id_val
            />

            {move || {
                if let Some(announcement) = selected_announcement.get() {
                    view! {
                        <ViewAnnouncementModal
                            show=viewing_announcement
                            set_show=set_viewing_announcement
                            announcement=announcement
                        />
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }
            }}
        </div>
    }.into_view()
}

#[component]
pub fn ViewAnnouncementModal(
    show: ReadSignal<bool>,
    set_show: WriteSignal<bool>,
    announcement: AnnouncementInfo,
) -> impl IntoView {
    view! {
        {move || if show.get() {
            view! {
                <div class="fixed inset-0 z-50 overflow-auto bg-black/50 flex items-center justify-center">
                    <div class="relative bg-white rounded-xl shadow-2xl max-w-3xl w-full m-4">
                        // Header section with title and close button
                        <div class="flex items-center justify-between border-b border-gray-200 px-6 py-4">
                        <h2 class="text-2xl font-bold text-gray-800 flex items-center gap-2">
                          <AnnouncementMicAlt size="1em" />
                            <span>Announcement</span>
                        </h2>
                        <button
                                class="p-2 hover:bg-gray-100 rounded-full transition-colors"
                                on:click=move |_| set_show.set(false)
                            >
                                <svg class="w-6 h-6 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                                </svg>
                            </button>
                        </div>

                        // Main content section
                        <div class="px-6 py-4">
                            // Announcement title with indicator
                            <div class="flex items-center gap-3 mb-4">
                                <div class="w-2 h-2 bg-customBlue rounded-full"></div>
                                <h3 class="text-xl font-semibold text-gray-800">
                                    {announcement.title.clone()}
                                </h3>
                            </div>

                            // Metadata (timestamp)
                            <div class="flex items-center gap-2 text-sm text-gray-500 mb-6">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                          d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                </svg>
                                <span>
                                    {"Posted on "}
                                    {announcement.time.format("%B %d, %Y at %I:%M %p").to_string()}
                                </span>
                            </div>

                            // Content
                            <div class="bg-gray-50 rounded-lg p-6">
                                <div class="prose max-w-none">
                                    <p class="whitespace-pre-wrap text-gray-700 leading-relaxed">
                                        {announcement.contents.clone()}
                                    </p>
                                </div>
                            </div>
                        </div>

                        // Footer
                        <div class="border-t border-gray-200 px-6 py-4 bg-gray-50 rounded-b-xl">
                            <button
                                class="w-full px-4 py-2 bg-customBlue hover:bg-customBlue-HOVER text-white rounded-lg transition-colors text-sm font-medium"
                                on:click=move |_| set_show.set(false)
                            >
                                "Close Announcement"
                            </button>
                        </div>
                    </div>
                </div>
            }
        } else {
            view! { <div></div> }
        }}
    }
}
