use super::push_notifications::get_authenticated_user;
use crate::data::database::class_functions::{get_class_name, get_users_classes};
/**
 * Component view and logic for the header at the top of the page of the site
 */
use crate::data::database::user_functions::Logout;
use crate::data::global_state::{Authentication, User};
use crate::pages::view_class_posts::class::ClassId;
use crate::resources::images::svgs::announcement_bell::AnnouncementBell;
use crate::resources::images::svgs::dashboard_icon::DashboardIcon;
use crate::resources::images::svgs::drop_down_bars::DropDownBars;
use crate::resources::images::svgs::drop_down_bars_close::DropDownBarsCloseIcon;
use crate::resources::images::svgs::logout_icon::LogoutIcon;
use crate::resources::images::svgs::profile_icon::ProfileIcon;
use crate::resources::images::svgs::settings_icon::SettingsIcon;
use crate::{
    app::expect_auth_context, data::database::announcement_functions::get_announcement_list,
};
use leptos::*;
use leptos_router::{ActionForm, A};

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
      <div class="flex justify-between items-center p-4 text-gray-600 bg-white border-b border-customBlue">
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
          <span class="flex items-center mr-4 text-xl font-bold">{first_name}</span>
          {move || {
            class_id()
              .map(|_| {
                view! {
                  <div class="relative group">
                    // <button class="pr-2">
                    //   <AnnouncementBell size="1.3rem" />
                    // </button>
                    // <span class="inline-flex items-baseline"></span>
                    <div class="absolute right-0 top-full invisible bg-white rounded-lg shadow-md group-hover:visible group-hover:opacity-100 group-hover:scale-100 z-[9999] mt-[-0.1rem]">
                      <AnnouncementInfo />
                    </div>
                  </div>
                }
              })
          }}
          <div class="flex relative items-center group">
            <button
              class="p-2 bg-white rounded-md hover:bg-gray-100 focus:ring-2 focus:ring-gray-300 focus:outline-none"
              on:click=move |_| set_dropdown_visible(!dropdown_visible())
            >
              {move || {
                if dropdown_visible() {
                  view! { <DropDownBarsCloseIcon size="5rem" /> }
                } else {
                  view! { <DropDownBars size="1.3rem" /> }
                }
              }}
            </button>
            <div class=move || {
              let visibility_classes = if dropdown_visible.get() {
                "visible opacity-100 scale-100"
              } else {
                "invisible opacity-0 scale-95"
              };
              format!("{} {}", base_classes, visibility_classes)
            }>
              <ul class="py-1 w-36 text-lg text-gray-700 rounded-md">
                <li class="py-2 px-4 cursor-pointer hover:bg-gray-100">
                  <a href="/profile" class="flex gap-2 items-center w-full h-full">
                    <ProfileIcon size="1em" />
                    Profile
                  </a>
                </li>
                <li class="py-2 px-4 cursor-pointer hover:bg-gray-100">
                  <a href="/settings" class="flex gap-2 items-center w-full h-full">
                    <SettingsIcon size="1em" />
                    Settings
                  </a>
                </li>
                <li class="py-2 px-4 cursor-pointer hover:bg-gray-100">
                  <a
                    href="/classes"
                    class="flex gap-2 items-center w-full h-full"
                    aria-current="page"
                    style="all: unset; display: flex; align-items: center; gap: 0.5rem;"
                  >
                    <DashboardIcon size="1em" />
                    Dashboard
                  </a>
                </li>
                <li class="py-1 px-4 cursor-pointer hover:bg-gray-100">
                  <ActionForm action=logout_action class="w-full">
                    <div class="flex gap-2 items-center">
                      <LogoutIcon size="1em" />
                      <input
                        class="p-0 m-0 text-left bg-transparent border-none cursor-pointer hover:bg-transparent focus:outline-none"
                        type="submit"
                        value="Logout"
                      />
                    </div>
                  </ActionForm>
                </li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    }
}

#[component]
pub fn AnnouncementInfo() -> impl IntoView {
    let announcements = create_resource(
        || (),
        |_| async move {
            get_x_newest_announcements_for_user()
                .await
                .unwrap_or_default()
        },
    );

    view! {
      <ul
        class="py-1 mx-1 w-40 text-lg text-left text-gray-700 z-[9999]"
        style="position: relative;"
      >
        <Suspense fallback=move || {
          view! { <li class="py-2 px-4 cursor-pointer hover:bg-gray-100">"Loading..."</li> }
        }>
          {announcements()
            .map(|announcement_info_vec| {
              announcement_info_vec
                .into_iter()
                .rev()
                .take(3)
                .map(|announcement_info| {
                                let class_name = create_resource(
                                    move || announcement_info.0,
                                    |class_id| async move {
                                        get_class_name_with_id(class_id).await
                                    },
                                );
                  view! {
                    <li class="py-2 px-4 cursor-pointer hover:bg-gray-100">
                      <A
                        href=format!(
                          "/classes/{}/announcement/{}",
                          announcement_info.0,
                          announcement_info.1,
                        )
                        class="block"
                      >
                        <Suspense fallback=move || {
                          view! { <div>"Loading class name..."</div> }
                        }>
                            {class_name()
                                .map(|class_name| {
                                    view! { <div class="font-bold">{class_name}</div> }
                                })
                            }
                      </Suspense>
                        {announcement_info.2}
                      </A>
                    </li>
                  }
                })
                .collect_view()
            })}
        </Suspense>
      </ul>
    }
}

pub async fn get_class_name_with_id(class_id: i32) -> String {
    get_class_name(class_id)
        .await
        .unwrap_or_else(|_| "Class not found".to_string())
}

/// Function to get the x newest announcement titles and contents from all classes a user is enrolled in
pub async fn get_x_newest_announcements_for_user() -> Result<Vec<(i32, i32, String)>, ServerFnError>
{
    // Get authenticated user
    let user = match get_authenticated_user() {
        Ok(user) => user,
        Err(err) => {
            logging::log!("User not authenticated. Redirecting to login...");
            return Err(ServerFnError::ServerError(
                err.as_string()
                    .unwrap_or_else(|| "Unknown error".to_string()),
            ));
        }
    };

    let count = 3;
    let classes = get_users_classes(user.id, user.role.clone()).await?;
    let mut all_announcements: Vec<(i32, i32, String)> = Vec::new();

    for class in classes {
        let announcements = get_announcement_list(class.id).await?;

        for announcement in announcements {
            all_announcements.push((
                class.id,
                announcement.announcement_id,
                announcement.title.clone(),
            ));
        }
    }

    // Sort all announcements by time in descending order
    all_announcements.sort_by(|a, b| b.1.cmp(&a.1));

    // Take the top x newest announcements
    let newest_announcements = all_announcements.into_iter().take(count).collect();

    Ok(newest_announcements)
}
