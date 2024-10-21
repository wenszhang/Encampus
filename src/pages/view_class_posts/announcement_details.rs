use crate::data::database::announcement_functions::get_announcement_by_id;
use crate::data::global_state::GlobalState;
use leptos::*;
use leptos_router::{use_params, Params};

#[derive(Params, PartialEq, Clone)]
pub struct AnnouncementId {
    pub announcement_id: i32,
}

#[component]
pub fn AnnouncementDetails() -> impl IntoView {
    // Get URL parameters
    let announcement_id_result = use_params::<AnnouncementId>();
    let global_state = expect_context::<GlobalState>();

    let is_instructor = move || global_state.role.get() == Some("Instructor".to_string());

    let announcement = create_resource(
        move || {
            announcement_id_result
                .get()
                .ok()
                .map(|id| id.announcement_id)
        },
        |announcement_id| async move {
            if let Some(id) = announcement_id {
                get_announcement_by_id(id).await.ok()
            } else {
                None
            }
        },
    );

    view! {
        <div class="announcement-details">
            <div class=format!("bg-[#EEEEEE] rounded-xl")>
            <Suspense fallback=|| view! { <p>{"Loading announcement..."}</p> }>
                {move || match announcement.get() {
                    None => view! {}.into_view(),
                    Some(None) => view! { <p>{"Announcement not found."}</p> }.into_view(),
                    Some(Some(announcement_details)) => view! {
                        <div class="p-2 border-b border-gray-300">
                            <h4 class="font-bold">{announcement_details.title.clone()}</h4>
                            <p class="text-sm">{announcement_details.contents.clone()}</p>
                            <p class="text-xs text-gray-500">
                                {announcement_details.time.format("%Y-%m-%d %H:%M:%S").to_string()}
                            </p>
                            {is_instructor().then(|| view! {
                                // TODO instructor edit post
                                // <p class="text-green-600">{"You are an instructor."}</p>
                            })}
                        </div>
                    }.into_view(),
                }}
            </Suspense>
            </div>
        </div>
    }
}
