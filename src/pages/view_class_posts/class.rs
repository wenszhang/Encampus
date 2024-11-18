use super::question_tile::QuestionTile;
use crate::data::database::announcement_functions::get_announcement_list;
use crate::data::database::class_functions::get_class_name;
use crate::data::database::post_functions::get_posts;
use crate::data::database::post_functions::get_search_posts;
use crate::data::database::post_functions::Post;
use crate::data::database::post_functions::PostFetcher;
use crate::expect_logged_in_user;
use crate::pages::global_components::announcements::Announcements;
use crate::pages::global_components::header::Header;
use crate::pages::global_components::sidebar::Sidebar;
use crate::pages::view_class_posts::create_post::CreatePost;
use crate::pages::view_class_posts::edit_post::EditPost;
use crate::resources::images::svgs::filter_icon::FilterIcon;
use crate::resources::images::svgs::information_icon::InformationIcon;
use crate::resources::images::svgs::magnifying_glass::MagnifyingGlass;

use leptos::*;
use leptos_router::{use_params, Outlet, Params};
use once_cell::sync::Lazy;

#[derive(Params, PartialEq, Clone)]
pub struct ClassId {
    pub class_id: i32,
}

#[derive(Params, PartialEq, Clone)]
pub struct FilterKeywords {
    keywords: String,
}

pub static IS_DISPLAYED_EDIT: Lazy<RwSignal<bool>> = Lazy::new(|| create_rw_signal(false));
/**
 * Page getting and displaying all posts in a class
 */
#[component]
pub fn ClassPage() -> impl IntoView {
    let (user, _) = expect_logged_in_user!();

    // Fetch class id from route in the format of "class/:class_id"
    let class_id = use_params::<ClassId>();
    let (post_list, set_posts) = create_signal::<Vec<Post>>(vec![]);
    let (filter_keywords, set_filter_keywords) = create_signal("".to_string());

    let on_input = |setter: WriteSignal<String>| {
        move |ev| {
            setter(event_target_value(&ev));
        }
    };

    let post_data = PostFetcher {
        class_id: class_id.get().unwrap().class_id,
        user_id: user().id,
    };
    let posts = create_resource(
        move || (post_data),
        |post_data| async move {
            get_posts(post_data.class_id, post_data.user_id)
                .await
                .unwrap_or_default()
        },
    );
    provide_context(posts);

    let filtered_posts_action = create_action(move |_| async move {
        if let Ok(new_posts) = get_search_posts(
            class_id.get().unwrap().class_id,
            user().id,
            filter_keywords.get(),
        )
        .await
        {
            set_posts(new_posts);
        }
    });

    let class_name = create_local_resource(class_id, |class_id| async {
        get_class_name(class_id.unwrap().class_id)
            .await
            .unwrap_or_else(|_| "Failed".to_string())
    });

    let announcements = create_resource(
        move || class_id.get().map(|id| id.class_id),
        |class_id| async move {
            get_announcement_list(class_id.unwrap())
                .await
                .unwrap_or_else(|_| vec![])
        },
    );

    // TODO: Use signal to store the question title when clicking a tile
    let (question_title, _set_question_title) = create_signal("".to_string());

    // Reactively update the document title when class_name or question_title changes.
    create_effect(move |_| {
        let current_class_name = class_name().unwrap_or_else(|| "Unknown Class".to_string());
        // Only show question title if it is not empty
        let title = if question_title().is_empty() {
            current_class_name
        } else {
            format!("{} - {}", current_class_name, question_title())
        };
        leptos_dom::document().set_title(&title);

        if let Some(fetched_posts) = posts.get() {
            set_posts(fetched_posts.clone()); // Set the signal to the fetched posts
        }
    });

    let (is_visible, set_is_visible) = create_signal(false);

    view! {
      <div class="flex">
        <Sidebar />
        <div class="flex-1">
          <Suspense fallback=move || view! {}>
            <Header
              text=class_name().unwrap_or_default()
              logo=None
              class_id=(move || class_id().ok().map(|id| id.class_id)).into_signal()
            />
          </Suspense>
          <span class="inline-flex items-baseline ml-5">
            <button class="pt-7 pr-1">
              <InformationIcon size="20px" />
            </button>
            <h3 class="pb-1 text-s">"Help"</h3>
          </span>
          <div class="flex justify-center pt-8 mx-20">
            <div class="flex justify-center items-center">
              <div class="flex relative items-center p-2 bg-white rounded-full border border-gray-300 shadow-lg focus-within:border-blue-500 w-[35rem]">
                <input
                  type="text"
                  placeholder="Search posts by keywords..."
                  class="pr-24 pl-5 w-full bg-white border-none focus:outline-none"
                  on:input=on_input(set_filter_keywords)
                  on:keydown=move |ev: web_sys::KeyboardEvent| {
                    if ev.key() == "Enter" {
                      if filter_keywords.get() != "" {
                        filtered_posts_action.dispatch(filter_keywords.get());
                      } else {
                        set_posts(posts.get().unwrap());
                      }
                    }
                  }
                  prop:value=filter_keywords
                />
                <button
                  class="flex absolute inset-y-0 top-1 right-12 justify-between items-center py-1 px-10 text-white bg-gray-300 rounded-full hover:bg-gray-400"
                  style="height: 30px;"
                  on:click=move |_| {
                    filtered_posts_action.dispatch(filter_keywords.get());
                  }
                >
                  <p class="pr-2 text-xs">"Filter Posts"</p>
                  <FilterIcon size="20px" />
                </button>
                <button class="flex absolute inset-y-0 right-0 items-center pr-4">
                  <MagnifyingGlass size="21px" />
                </button>
              </div>
            </div>
            <button
              class="py-2 px-4 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue"
              on:click=move |_| set_is_visible(!is_visible())
            >
              {move || if is_visible() { "Cancel" } else { "Post +" }}
            </button>
          </div>
          <div class="flex flex-col gap-4 my-10 mx-20 align">
            <Show when=is_visible fallback=|| ()>
              <CreatePost on_new_post=move || set_is_visible(false) />
            </Show>
            // <Show when=move || IS_DISPLAYED_EDIT.get() fallback=|| ()>
            // <EditPost />
            // </Show>
            // Gets replaced with the focused post if there's one in the route. See router
            <Outlet />
            // announcements section
            <Suspense fallback=move || {
              view! { <p>"Loading announcements..."</p> }
            }>
              {move || {
                let ann_list = announcements().unwrap_or_default();
                view! { <Announcements announcements=ann_list /> }
              }}
            </Suspense>
            <div class="grid grid-cols-3 gap-4">
              <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                <For each=move || post_list.get() key=|post| post.post_id let:post>
                  {
                    let private = post.private;
                    post
                      .resolved
                      .then(|| {
                        view! {
                          <QuestionTile
                            post=post.clone()
                            is_resolved=(|| false).into_signal()
                            is_private=(move || private).into_signal()
                          />
                        }
                      })
                      .unwrap_or_else(|| {
                        view! {
                          <QuestionTile
                            post=post.clone()
                            is_resolved=(|| true).into_signal()
                            is_private=(move || private).into_signal()
                          />
                        }
                      })
                  }
                </For>
              </Suspense>
            </div>
          </div>
        </div>
      </div>
    }.into_view()
}
