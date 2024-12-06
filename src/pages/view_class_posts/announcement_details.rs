use crate::data::database::announcement_functions::get_announcement_by_id;
use leptos::*;
use leptos_router::{use_params, Params};

#[derive(Params, PartialEq, Clone)]
pub struct AnnouncementId {
    pub announcement_id: i32,
}

#[component]
pub fn AnnouncementDetails() -> impl IntoView {
    // Get URL parameters
    let announcement_id = {
        let announcement_id_result = use_params::<AnnouncementId>();
        move || announcement_id_result().expect("Tried to render announcement details page without announcement id").announcement_id
    };

    let announcement = create_resource(
        announcement_id,
        |announcement_id| async move {get_announcement_by_id(announcement_id).await.unwrap()}
    );

    view! {
      <div class="announcement-details">
        <div class=format!("bg-[#EEEEEE] rounded-xl")>
          <Suspense fallback=|| {
            view! { <p>{"Loading announcement..."}</p> }
          }>
            {move || announcement().map(|announcement_details| 
              view! {
                <div class="p-2 border-b border-gray-300">
                  <h4 class="font-bold">{announcement_details.title.clone()}</h4>
                  <p class="text-sm">{announcement_details.contents.clone()}</p>
                  <p class="text-xs text-gray-500">
                    {announcement_details.time.format("%Y-%m-%d %H:%M:%S").to_string()}
                  </p>
                </div>
              })
            }
          </Suspense>
        </div>
      </div>
    }
}
