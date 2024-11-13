use crate::data::database::announcement_functions::{
    post_announcement, AddAnnouncementInfo, AnnouncementInfo,
};
use crate::data::database::class_functions::check_user_is_instructor;
use crate::expect_logged_in_user;
use crate::pages::view_class_posts::class::ClassId;
use crate::resources::images::svgs::announcement_mic::AnnouncementMic;
use leptos::*;
use leptos_router::{use_params, A};

//TODO handle too many announcements and href to announcements page
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

    let on_input = |setter: WriteSignal<String>| {
        move |ev| {
            setter(event_target_value(&ev));
        }
    };

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
      <div class="flex overflow-hidden relative flex-col rounded-lg shadow-lg">
        // Announcement header
        <div class="flex justify-between items-center px-3 w-full h-7 rounded-t-lg bg-customBlue">
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

        // Announcement info
        <div class=format!(
          "bg-[#EEEEEE]{}",
          "flex flex-col",
        )>
          {move || {
            let announcements_clone = sorted_announcements.clone();
            if is_expanded.get() {
              view! {
                <ul>
                  {announcements_clone
                    .into_iter()
                    .map(|announcement| {
                      view! {
                        <li class="p-2 border-b border-gray-300 hover:bg-gray-100">
                          <A
                            href=format!(
                              "/classes/{}/announcement/{}",
                              class_id_val,
                              announcement.announcement_id,
                            )
                            class="block"
                          >
                            <h4 class="font-bold">{announcement.title.clone()}</h4>
                            <p class="text-sm">{announcement.contents.clone()}</p>
                            <p class="text-xs text-gray-500">
                              {announcement.time.format("%Y-%m-%d %H:%M:%S").to_string()}
                            </p>
                          </A>
                        </li>
                      }
                    })
                    .collect::<Vec<_>>()}
                </ul>
              }
            } else {
              view! { <ul></ul> }
            }
          }}
          <Suspense fallback=|| {
            view! { <p>{"Loading ..."}</p> }
          }>
            {move || {
              if is_instructor().unwrap_or_default() {
                view! {
                  <div class="flex flex-col p-4">
                    <button
                      class="px-3 my-1 ml-auto text-white rounded-full bg-customBlue hover:bg-customBlue-HOVER"
                      on:click=move |_| set_is_adding_post.update(|v| *v = !*v)
                    >
                      {move || if is_adding_post.get() { "Cancel" } else { "Add New Announcement" }}
                    </button>
                    {move || {
                      if is_adding_post.get() {
                        view! {
                          <div class="flex flex-col">
                            <input
                              class="p-2 mb-2 rounded border border-gray-300"
                              type="text"
                              placeholder="Announcement Title"
                              prop:value=title
                              on:input=on_input(set_title)
                            />
                            <textarea
                              class="p-2 mb-2 rounded border border-gray-300"
                              placeholder="Announcement Contents"
                              prop:value=contents
                              on:input=on_input(set_contents)
                            />
                            <button
                              class="py-1 px-3 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue"
                              on:click=move |_| {
                                let new_announcement = AddAnnouncementInfo {
                                  title: title.get(),
                                  contents: contents.get(),
                                  class_id: class_id_val,
                                };
                                add_announcement_action.dispatch(new_announcement);
                              }
                            >
                              "Post Announcement"
                            </button>
                          </div>
                        }
                      } else {
                        view! { <div></div> }
                      }
                    }}
                  </div>
                }
              } else {
                view! { <div></div> }
              }
            }}
          </Suspense>

        </div>
      </div>
    }.into_view()
}
