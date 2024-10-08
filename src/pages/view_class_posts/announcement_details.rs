use leptos::*;
use leptos_router::{use_params, Params};
use crate::pages::view_class_posts::class::ClassId;
use crate::data::database::announcement_functions::{get_announcement_by_id, AnnouncementInfo};
use crate::data::global_state::GlobalState;

#[derive(Params, PartialEq, Clone)]
pub struct AnnouncementId {
    pub announcement_id: i32,
}

#[component]
pub fn AnnouncementDetails() -> impl IntoView {
    // Get URL parameters
    let announcement_id_result = use_params::<AnnouncementId>();
    let class_id_result = use_params::<ClassId>();
    let class_id_val = class_id_result.get_untracked().unwrap().class_id;
    let global_state = expect_context::<GlobalState>();

    let is_instructor = move || global_state.role.get() == Some("instructor".to_string());

    let announcement = create_resource(
        move || announcement_id_result.get().ok().map(|id| id.announcement_id),
        |announcement_id| async move {
            if let Some(id) = announcement_id {
                get_announcement_by_id(id).await.ok()
            } else {
                None
            }
        },
    );

    // Create the view
    view! {
        <div class="announcement-details">
            <h2>{"Announcement Details"}</h2>

            {move || match announcement.get() {
                None => view! { <p>{"Loading announcement..."}</p> }.into_view(),
                Some(None) => view! { <p>{"Announcement not found."}</p> }.into_view(),
                Some(Some(announcement_details)) => view! {
                    <div class="p-2 border-b border-gray-300">
                        <h4 class="font-bold">{announcement_details.title.clone()}</h4>
                        <p class="text-sm">{announcement_details.contents.clone()}</p>
                        <p class="text-xs text-gray-500">
                            {announcement_details.time.format("%Y-%m-%d %H:%M:%S").to_string()}
                        </p>
                        {is_instructor().then(|| view! {
                            <p class="text-green-600">{"You are an instructor."}</p>
                        })}
                    </div>
                }.into_view(),
            }}
        </div>
    }
}
