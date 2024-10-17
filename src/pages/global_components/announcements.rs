use crate::data::database::announcement_functions::{
    post_announcement, AddAnnouncementInfo, AnnouncementInfo,
};
use crate::data::global_state::GlobalState;
use crate::resources::images::svgs::announcement_mic::AnnouncementMic;
use leptos::*;
use leptos_router::{use_params, A};
use crate::pages::view_class_posts::class::ClassId;

#[component]

//TODO handle too many announcements and href to announcements page
pub fn Announcements(announcements: Vec<AnnouncementInfo>) -> impl IntoView {
    let global_state: GlobalState = expect_context::<GlobalState>(); // Access global state
    let class_id = use_params::<ClassId>();
    let class_id_val = class_id.get_untracked().unwrap().class_id;
    let is_instructor = move || global_state.role.get() == Some("instructor".to_string());

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
            match post_announcement(announcementInfo, global_state.id.get_untracked().unwrap())
                .await
            {
                Ok(_announcement) => {}
                Err(_) => logging::error!("Attempt to post post failed. Please try again"),
            }
        }
    });

    view! {
      <div class="flex overflow-hidden relative flex-col rounded-lg shadow-lg bg-card-bg">
        // Announcement header
        <div class="flex justify-between items-center px-3 w-full h-7 rounded-t-lg bg-customBlue">
          <div class="flex items-center text-white">
            <AnnouncementMic size="5em" />
            <h3 class="px-2">"RECENT ANNOUNCEMENTS"</h3>
          </div>

          <div class="flex items-center text-white hover:bg-gray-300 hover:text-customBlue-HOVER">
            <button on:click=move |_| set_is_expanded.update(|v| *v = !*v)>
              <details open=is_expanded.get()>
                <summary>{move || if is_expanded.get() { "COLLAPSE" } else { "EXPAND" }}</summary>
              </details>
            </button>
          </div>
        </div>

      {move || {
                if is_instructor() {
                    view! {
                        <div class="flex flex-col p-4">
                            <input
                                class="mb-2 p-2 border border-gray-300 rounded"
                                type="text"
                                placeholder="Announcement Title"
                                prop:value=title
                                on:input=on_input(set_title)
                            />
                            <textarea
                                class="mb-2 p-2 border border-gray-300 rounded"
                                placeholder="Announcement Contents"
                                prop:value=contents
                                on:input=on_input(set_contents)
                            />
                            <button
                                class="bg-customBlue hover:bg-customBlue-HOVER text-white py-1 px-3 rounded-full focus:outline-none focus:ring-2 focus:ring-offset-customBlue focus:ring-offset-2"
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
                          <A href={format!("/classes/{}/announcement/{}", class_id_val, announcement.announcement_id)} class="block">
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
        </div>
      </div>
    }
}
