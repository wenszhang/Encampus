/**
 * QuestionTile component, displaying a tile for one post
 */
use crate::data::database::class_functions::check_user_is_instructor;
use crate::data::database::post_functions::{
    bump_post, endorse_post, remove_post, Post, PostFetcher,
};
use crate::expect_logged_in_user;
use crate::pages::global_components::notification::{
    NotificationComponent, NotificationDetails, NotificationType,
};
use crate::pages::view_class_posts::class::ClassId;
use crate::pages::view_class_posts::focused_post::get_post_details;
use crate::resources::images::svgs::bump_icon::BumpIcon;
use crate::resources::images::svgs::check_icon::CheckIcon;
use crate::resources::images::svgs::dots_icon::DotsIcon;
use crate::resources::images::svgs::endorsed_icon::EndorsedIcon;
use crate::resources::images::svgs::lock_icon::LockIcon;
use crate::resources::images::svgs::paper_icon::PaperIcon;
use crate::resources::images::svgs::remove_icon::RemoveIcon;
use crate::resources::images::svgs::unresolved_icon::UnresolvedIcon;

use ev::MouseEvent;
use leptos::*;
use leptos_dom::logging::console_debug_warn;
use leptos_router::{use_params, A};

enum TagPillProperties {
    Unresolved,
    Private,
    Endorsed,
    Resolved,
}

#[component]
pub fn DropDownMenu(
    post_id: i32,
    post_author_id: i32,
    set_endorsed: WriteSignal<bool>,
    is_endorsed: ReadSignal<bool>,
    //remove_action: Action<PostId, ()>,
) -> impl IntoView {
    let posts: Resource<PostFetcher, Vec<Post>> =
        expect_context::<Resource<PostFetcher, Vec<Post>>>();
    let (user, _) = expect_logged_in_user!();
    let class_id = use_params::<ClassId>();
    let is_on_my_post = move || user().id == post_author_id;
    let is_instructor = create_resource(class_id, move |class_id| {
        let user_id = user().id;
        async move {
            check_user_is_instructor(user_id, class_id.unwrap().class_id)
                .await
                .unwrap_or(false)
        }
    });
    let (notification_details, set_notification_details) =
        create_signal(None::<NotificationDetails>);

    // logging::log!("Global State: {:?}", global_state);

    let _notification_view = move || {
        notification_details.get().map(|details| {
            view! {
              <NotificationComponent
                notification_details=details.clone()
                on_close=move || set_notification_details(None)
              />
            }
        })
    };

    // Bump logic
    let bump_action = create_action(move |post_id: &i32| {
        let post_id = post_id.to_owned();

        async move {
            if let Err(e) = bump_post(post_id).await {
                eprintln!("Failed to bump post: {:?}", e);
                return;
            }
            posts.update(|posts| {
                if let Some(index) = posts
                    .as_mut()
                    .unwrap()
                    .iter()
                    .position(|post| post.post_id == post_id)
                {
                    let bumped_post = posts.as_mut().unwrap().remove(index);

                    let insert_position = posts
                        .as_mut()
                        .unwrap()
                        .iter()
                        .position(|post| post.post_id == post_id)
                        .unwrap_or(posts.as_ref().unwrap().len());

                    posts.as_mut().unwrap().insert(insert_position, bumped_post);

                    if insert_position > 0 {
                        posts.as_mut().unwrap().swap(insert_position, 0);
                    }
                }
            });
        }
    });

    // Endorsed logic
    let endorsed_action = create_action(move |(post_id, status): &(i32, bool)| {
        let post_id = post_id.to_owned();
        let status = status.to_owned();

        async move {
            match endorse_post(post_id, status).await {
                Ok(()) => {
                    // Successfully endorsed the post
                    set_endorsed(status);
                }
                Err(_) => {
                    logging::error!("Attempt to endorse post failed. Please try again");
                    set_notification_details(Some(NotificationDetails {
                        message: "Failed to endorse post. Please try again.".to_string(),
                        notification_type: NotificationType::Error,
                    }));
                }
            }
        }
    });

    // Remove logic
    let remove_action = create_action(move |post_id: &i32| {
        let post_id = post_id.to_owned();

        async move {
            match get_post_details(post_id).await {
                Ok(current_post) => {
                    if let Ok(()) = remove_post(post_id, user().id).await {
                        posts.update(|posts| {
                            if let Some(index) = posts
                                .as_mut()
                                .unwrap()
                                .iter()
                                .position(|post| post.post_id == current_post.0.post_id)
                            {
                                posts.as_mut().unwrap().remove(index);
                            }
                        });
                    }
                }
                Err(_) => {
                    logging::error!("Attempt to remove post failed. Please try again");
                    set_notification_details(Some(NotificationDetails {
                        message: "Failed to remove post. Please try again.".to_string(),
                        notification_type: NotificationType::Error,
                    }));
                }
            }
        }
    });

    view! {
      <div class="pr-2 text-right">
        {move || match is_instructor() {
          Some(true) => {
            logging::log!("User is an instructor");
            // Log to verify
            view! {
              <div class="p-1">
                <button
                  class="inline-flex items-center p-1 w-full text-sm leading-tight text-gray-700 rounded-md hover:text-black hover:bg-gray-100"
                  on:click=move |_| endorsed_action.dispatch((post_id, !is_endorsed()))
                >
                  <EndorsedIcon size="20px" />
                  <span class="ml-2">Endorse</span>
                </button>
              </div>
            }
              .into_view()
          }
          Some(false) => {
            logging::log!("User is not an instructor");
            view! {
              <div class="p-1">
                <button
                  class="inline-flex items-center p-1 w-full text-sm leading-tight text-gray-700 rounded-md hover:text-black hover:bg-gray-100"
                  on:click=move |_| bump_action.dispatch(post_id)
                >
                  <BumpIcon size="20px" />
                  <span class="ml-2">bump</span>
                </button>
              </div>
            }
              .into_view()
          }
          None => {
            logging::log!("Checking instructor status...");
            view! {
              <div class="p-1 text-gray-500">
                <span>"Checking instructor status..."</span>
              </div>
            }
              .into_view()
          }
        }}
        {move || {
          if is_on_my_post() {
            Some(
              view! {
                <div class="p-1">
                  <button
                    class="inline-flex items-center p-1 w-full text-sm leading-tight text-red-500 rounded-md hover:text-black hover:bg-gray-100"
                    on:click=move |_| remove_action.dispatch(post_id)
                  >
                    <RemoveIcon size="20px" />
                    <span class="ml-2">Remove</span>
                  </button>
                </div>
              },
            )
          } else {
            None
          }
        }}
      </div>
    }.into_view()
}

#[component]
pub fn QuestionTile(
    post: Post,
    is_resolved: Signal<bool>,
    is_private: Signal<bool>,
) -> impl IntoView {
    let (menu_invisible, set_menu_invisible) = create_signal(true);
    let (is_endorsed, set_endorsed) = create_signal(post.endorsed);
    // let (is_pinned, set_is_pinned) = create_signal(!post.pinned); // Retrieve pin state from post data

    let toggle_menu = move |e: MouseEvent| {
        e.stop_propagation();
        set_menu_invisible.update(|visible| *visible = !*visible);
    };

    (move || console_debug_warn(format!("debug {}", is_endorsed()).as_str()))();
    view! {
      <div
        class="relative h-60 rounded-lg shadow-lg transition-transform duration-300 hover:shadow-xl hover:scale-105 bg-card-bg"
        class=("border-4", move || is_endorsed())
        class=("border-customYellow", move || is_endorsed())
        class=("bg-customRed", move || is_resolved())
        class=("hover:bg-customRed-HOVER", move || is_resolved())
        class=("hover:bg-gray-200", move || !is_resolved())
        class=("bg-gray-100", move || !is_resolved())
      >

        <A href=format!("{}", post.post_id) class="block w-full h-full">
          <div
            class="flex flex-col justify-between items-center p-6 m-auto h-full text-lg font-semibold"

            // class:border-purple-500=is_private()

            // class:border-4=is_private()
            class=is_private()
          >

            // Card header
            <div class="flex top-0 left-0 z-10 gap-2 items-center pl-2 w-full h-12 text-xs rounded-t-lg shadow-md bg-card-header">
              {move || {
                if is_resolved() {
                  Some(view! { <TagPill props=TagPillProperties::Unresolved /> })
                } else {
                  None
                }
              }}
              {move || {
                if is_private() {
                  Some(view! { <TagPill props=TagPillProperties::Private /> })
                } else {
                  None
                }
              }}
              {move || {
                if is_endorsed() {
                  Some(view! { <TagPill props=TagPillProperties::Endorsed /> })
                } else {
                  None
                }
              }}
              {move || {
                if !is_resolved() {
                  Some(view! { <TagPill props=TagPillProperties::Resolved /> })
                } else {
                  None
                }
              }}
            </div>

            // Card body
            <div class="flex justify-center items-center p-4 w-full h-full text-center sm:p-6 md:p-8 lg:p-12">
              <p class="text-base font-bold">{post.title}</p>
            </div>
          </div>
        </A>
        <div class="flex absolute top-1 right-2 z-20 items-center">
          <button on:click=toggle_menu class="rounded-lg bg-card-header hover:shadow-customInset">
            <DotsIcon size="36px" />
          </button>
          // Dropdown menu
          <div class=move || {
            if menu_invisible() {
              "hidden"
            } else {
              "absolute right-0 top-4 mt-2 w-30 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5 p-1"
            }
          }>
            <DropDownMenu
              post_id=post.post_id
              post_author_id=post.author_id
              set_endorsed=set_endorsed
              is_endorsed=is_endorsed
            />

          </div>
        </div>
      </div>
    }
}

#[component]
fn TagPill(props: TagPillProperties) -> impl IntoView {
    let sharedClassesAll = "px-2 py-1 rounded-full";
    let sharedClassesWithIcon = "flex gap-2";

    match props {
        TagPillProperties::Unresolved => view! {
          <div class=[sharedClassesAll, sharedClassesWithIcon, "bg-customRed text-red-600"]
            .join(" ")>
              <span class="relative top-[2px]">
                <UnresolvedIcon size="1em" />
              </span>
            "Unresolved"
          </div>
        },
        TagPillProperties::Private => view! {
          <div class=[sharedClassesAll, sharedClassesWithIcon, "bg-customPurple text-purple-600"]
            .join(" ")>
              <span class="relative top-[2px]">
                <LockIcon size="1em" />
              </span>
              "Private"
          </div>
        },
        TagPillProperties::Endorsed => view! {
            <div class=[sharedClassesAll, sharedClassesWithIcon, "bg-customYellow text-customYellow-details"]
                .join(" ")>
                <span class="relative top-[2px]">
                  <EndorsedIcon size="1em" />
                </span>
                "Instructor Approved"
            </div>
        },
        TagPillProperties::Resolved => view! {
            <div class=[sharedClassesAll, sharedClassesWithIcon, "bg-customGreen text-customGreen-details"]
                .join(" ")>
                <span class="relative top-[2px]">
                  <CheckIcon size="1em" />
                </span>
                "Resolved"
            </div>
        },
        // TagPillProperties::Custom(CustomTag { title }) => {
        //     view! { <div class=[sharedClassesAll, "bg-white text-gray-600"].join(" ")>{title}</div> }
        // }
    }
}
