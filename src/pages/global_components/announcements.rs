use crate::data::database::announcement_functions::{
    post_announcement, AddAnnouncementInfo, AnnouncementInfo,
};
use crate::data::database::class_functions::check_user_is_instructor;
use crate::expect_logged_in_user;
use crate::pages::view_class_posts::class::ClassId;
use crate::resources::images::svgs::announcement_mic::AnnouncementMic;
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

    let (is_adding_post, set_is_adding_post) = create_signal(false);
    let (is_expanded, set_is_expanded) = create_signal(true);
    let (title, set_title) = create_signal(String::new());
    let (contents, set_contents) = create_signal(String::new());

    let mut sorted_announcements = announcements.clone();
    sorted_announcements.sort_by(|a, b| b.time.cmp(&a.time));

    let add_announcement_action = create_action(move |announcementInfo: &AddAnnouncementInfo| {
        let announcementInfo = announcementInfo.clone();
        async move {
            match post_announcement(announcementInfo, user().id).await {
                Ok(_announcement) => {}
                Err(_) => logging::error!("Attempt to post post failed. Please try again"),
            }
        }
    });

    view! {
      <div class="flex overflow-hidden relative flex-col rounded-lg shadow-lg bg-white">
          // Announcement header
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
                        <Suspense fallback=|| {
                            view! { <p>{"Loading ..."}</p> }
                        }>
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
                                    }
                                } else {
                                    view! { <div></div> }
                                }
                            }}
                        </Suspense>
                        <div class="h-[370px] overflow-y-auto">
                            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 p-4 bg-white">
                                {announcements_clone
                                    .into_iter()
                                    .map(|announcement| {
                                        view! {
                                            <A
                                                href=format!(
                                                    "/classes/{}/announcement/{}",
                                                    class_id_val,
                                                    announcement.announcement_id,
                                                )
                                            >
                                                <div class="flex overflow-hidden relative flex-col justify-between p-6 h-60 bg-card-header rounded-lg shadow-lg transition-transform duration-300 hover:bg-gray-100 hover:shadow-xl hover:scale-105">
                                                    <div class="flex-1">
                                                        <h4 class="text-lg font-semibold mb-2">{announcement.title.clone()}</h4>
                                                        <p class="text-sm text-gray-600 line-clamp-3">{announcement.contents.clone()}</p>
                                                    </div>
                                                    <p class="text-xs text-gray-500 mt-2">
                                                        {announcement.time.format("%Y-%m-%d %H:%M:%S").to_string()}
                                                    </p>
                                                </div>
                                            </A>
                                        }
                                    })
                                    .collect::<Vec<_>>()}
                            </div>
                        </div>
                    </>
                }
            } else {
                view! {
                    <>
                        <div></div>
                    </>
                }
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
          </div>
  }.into_view()
}
