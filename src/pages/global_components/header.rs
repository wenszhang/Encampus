/**
 * Component view and logic for the header at the top of the page of the site
 */
use crate::data::database::user_functions::Logout;
use crate::data::global_state::{Authentication, User};
use crate::resources::images::svgs::announcement_bell::AnnouncementBell;
use crate::resources::images::svgs::drop_down_bars::DropDownBars;
use crate::{
    app::expect_auth_context, data::database::announcement_functions::get_announcement_list,
};
use leptos::*;
use leptos_router::ActionForm;

#[component]
pub fn AnnouncementInfo(class_id: impl Fn() -> i32 + 'static) -> impl IntoView {
    let announcements = create_resource(class_id, |class_id| async move {
        get_announcement_list(class_id).await.unwrap_or_default()
    });

    view! {
      <ul class="py-1 mx-1 w-40 text-lg text-left text-gray-700">
        <Suspense fallback=move || {
          view! { <li class="py-2 px-4 cursor-pointer hover:bg-gray-100">"Loading..."</li> }
        }>
          {announcements()
            .map(|announcement_info_vec| {
              announcement_info_vec
                .into_iter()
                .map(|announcement_info| {
                  view! {
                    <li class="py-2 px-4 cursor-pointer hover:bg-gray-100">
                      {announcement_info.title}
                    </li>
                  }
                })
                .collect_view()
            })}
        </Suspense>
      </ul>
    }
}

#[component]
pub fn Header(text: String, logo: Option<String>, class_id: Signal<Option<i32>>) -> impl IntoView {
    let logo_src = logo.as_deref().unwrap_or("images/BlockU_RGB.png");

    let authentication = expect_auth_context();

    let (first_name, _) = create_slice(
        authentication,
        |auth| {
            if let Authentication::Authenticated(User { first_name, .. }) = auth {
                Some(first_name.clone())
            } else {
                None
            }
        },
        |_, _: &String| todo!(), // Unimportant
    );

    let (dropdown_visible, set_dropdown_visible) = create_signal(false);

    let base_classes = "absolute right-0 top-full z-50 bg-white rounded-lg shadow-md transition-transform duration-200 ease-out";

    let header_text_href = move || {
        if let Some(id) = class_id() {
            format!("/classes/{}", id)
        } else {
            "/classes".to_string()
        }
    };

    let logout_action = create_server_action::<Logout>();

    view! {
      <div class="flex justify-between items-center p-4 text-gray-600 bg-white">
        <div class="flex items-center" style="padding: 0;">
          <a href="/classes">
            <img src=format!("/{}", logo_src) alt="Logo" class="h-8" style="padding: 0;" />
          </a>
          <a
            href=header_text_href
            class="flex items-center text-gray-600"
            style="font-size: 2rem; height: 2rem; line-height: 2rem; padding: 0;"
          >
            {text}
          </a>
        </div>

        <div class="flex items-center">
          {move || {
            class_id()
              .map(|class_id: i32| {
                view! {
                  <div class="relative group">
                    <span class="inline-flex items-baseline">
                      <h3 class="px-2">"Notifications"</h3>
                      <button class="pr-2">
                        <AnnouncementBell size="1.3rem" />
                      </button>
                    </span>
                    <div class="absolute right-0 top-full invisible bg-white rounded-lg shadow-md group-hover:visible group-hover:opacity-100 group-hover:scale-100 mt-[-0.1rem]">
                      <AnnouncementInfo class_id=move || class_id />
                    </div>
                  </div>
                }
              })
          }} <span class="flex items-center mr-4 text-xl font-bold">{first_name}</span>
          <div class="flex relative items-center group">
            <button on:click=move |_| set_dropdown_visible(!dropdown_visible())>
              <DropDownBars size="1.3rem" />
            </button>
            <div class=move || {
              let visibility_classes = if dropdown_visible.get() {
                "visible opacity-100 scale-100"
              } else {
                "invisible opacity-0 scale-95"
              };
              format!("{} {}", base_classes, visibility_classes)
            }>
              <ul class="py-1 w-36 text-lg text-gray-700">
                <li class="py-2 px-4 cursor-pointer hover:bg-gray-100">
                  <a href="/profile" class="block w-full h-full">
                    Profile
                  </a>
                </li>
                <li class="py-2 px-4 cursor-pointer hover:bg-gray-100">
                  <a href="/settings" class="block w-full h-full">
                    Settings
                  </a>
                </li>
                <li class="py-2 px-4 cursor-pointer hover:bg-gray-100">
                  <a
                    href="/classes"
                    class="block w-full h-full"
                    aria-current="page"
                    style="all: unset; display: block; text-align: left; color: inherit; font-size: inherit; font-weight: inherit; line-height: inherit;"
                  >
                    Dashboard
                  </a>
                </li>
                <li class="py-2 px-4 cursor-pointer hover:bg-gray-100">
                  <ActionForm action=logout_action>
                    <input class="block w-full h-full" type="submit" value="Logout" />
                  </ActionForm>
                </li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    }
}
