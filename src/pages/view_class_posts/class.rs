use std::collections::HashSet;
use super::question_tile::QuestionTile;
use crate::data::database::announcement_functions::get_announcement_list;
use crate::data::database::post_functions::get_posts;
use crate::data::database::post_functions::get_search_posts;
use crate::data::database::post_functions::Post;
use crate::data::database::post_functions::PostFetcher;
use crate::expect_logged_in_user;
use crate::pages::global_components::announcements::Announcements;
use crate::pages::global_components::header::Header;
use crate::pages::global_components::sidebar::Sidebar;
use crate::pages::view_class_posts::create_post::CreatePost;
use crate::resources::images::svgs::cancel_icon::CancelIcon;
use crate::resources::images::svgs::magnifying_glass::MagnifyingGlass;
use leptos::*;
use leptos_router::{use_params, Outlet, Params};

#[derive(Params, PartialEq, Clone, Default)]
pub struct ClassId {
    pub class_id: i32,
}

#[derive(Params, PartialEq, Clone)]
pub struct FilterKeywords {
    keywords: String,
}

/**
 * Page getting and displaying all posts in a class
 */
#[component]
pub fn ClassPage() -> impl IntoView {
    let (user, _) = expect_logged_in_user!();

    // Fetch class id from route in the format of "class/:class_id"
    let class_id = {
      let class_params = use_params::<ClassId>();
      move || class_params().expect("Tried to render class page without class id").class_id
    };
     
    let (is_visible, set_is_visible) = create_signal(false);

    let filter_input_node: NodeRef<html::Input> = NodeRef::new();

    let post_data = move || PostFetcher {
        class_id: class_id(),
        user_id: user().id,
    };
    let posts = create_resource(
      post_data,
        |post_data| async move {
            get_posts(post_data.class_id, post_data.user_id)
                .await
                .unwrap_or_default()
        },
    );
    provide_context(posts); // This is not great because resources should be very carefully managed and used in <suspense> or <transition> tags

    let filtered_post_ids_action = create_action(|(filter_string, class_id, user_id): &(String, i32, i32)| {
      let filter_keywords = filter_string.clone();
      let class_id = *class_id;
      let user_id = *user_id;
      async move {
        if filter_keywords.is_empty() {
            return None;
        }
        get_search_posts(
            class_id,
            user_id,
            filter_keywords,
        )
        .await.ok()
    }});

    let announcements = create_resource(
        class_id,
        |class_id| async move {
            get_announcement_list(class_id)
                .await
                .unwrap_or_else(|_| vec![])
        },
    );

    view! {
      <div class="flex">
        <Sidebar />
        <div class="flex-1">
          <Header />
          // <button class="pt-7 pr-1">
          // <InformationIcon size="20px" />
          // </button>
          // <h3 class="pb-1 text-s">"Help"</h3>
          <span class="inline-flex items-baseline ml-5"></span>
          <div class="flex justify-center pt-8 mx-20">
            <div class="flex justify-center items-center">
              <div class="flex relative items-center p-2 bg-white rounded-full border border-gray-300 shadow-xl focus-within:border-blue-500 w-[30rem]">
                <input
                  type="text"
                  placeholder="Search posts by keywords..."
                  class="py-1.5 pr-16 pl-4 w-full bg-white border-none focus:outline-none"
                  node_ref=filter_input_node
                  on:keydown=move |ev: web_sys::KeyboardEvent| {
                    let filter_keywords = event_target_value(&ev);
                    if ev.key() == "Enter" {
                      if filter_keywords.is_empty() {
                        filtered_post_ids_action.value().set(None);
                      } else {
                        filtered_post_ids_action.dispatch((filter_keywords, class_id(), user().id));
                      }
                    }
                  }
                />
                <button
                  class="flex absolute top-0 right-0 bottom-0 justify-center items-center bg-gradient-to-r rounded-r-full transition-all duration-200 w-[4rem] bg-[#AAAA] hover:bg-[#999999]"
                  on:click=move |_| {
                  filtered_post_ids_action.dispatch((filter_input_node.get().expect("filter input above this button should exist").value(), class_id(), user().id));
                  }
                >
                  <MagnifyingGlass size="2em" />
                </button>
              </div>
            </div>
            <button
              class=move || {
                if is_visible() {
                  "ml-4 py-2 px-4 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-red-500 hover:bg-red-600 focus:ring-offset-red-500 flex items-center gap-2"
                } else {
                  "ml-4 py-2 px-4 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue gap-2"
                }
              }
              on:click=move |_| set_is_visible(!is_visible())
            >
              {move || {
                if is_visible() {
                  view! {
                    <div class="flex gap-2 items-center">
                      <CancelIcon size="1em" />
                      "Cancel"
                    </div>
                  }
                } else {
                  view! { <div class="flex gap-2 items-center">"Post +"</div> }
                }
              }}
            </button>
          </div>
          <div class="flex flex-col gap-4 my-10 mx-20 align">
            <Show when=is_visible fallback=|| ()>
              <CreatePost on_new_post=move || set_is_visible(false) />
            </Show>
            // Gets replaced with the focused post if there's one in the route. See router
            <Outlet />


            // announcements section
            // <Transition fallback=move || {
            //   view! { <p>"Loading announcements..."</p> }
            // }>
            //   {move || {
            //     let ann_list = announcements().unwrap_or_default();
            //     view! {
            //       <Announcements
            //         announcements=ann_list
            //         class_id=move || class_id()
            //       />
            //     }
            //   }}
            // </Transition>


            <div class="grid grid-cols-3 gap-4">
              <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <FilteredPostsGrid
                  unfiltered_posts=(move || posts().unwrap_or_default()).into_signal()
                  filtered_ids=(move || filtered_post_ids_action.value()().flatten()).into_signal()
                />
              </Transition>
            </div>
          </div>
        </div>
      </div>
    }.into_view()
}

#[component]
fn FilteredPostsGrid(
    unfiltered_posts: Signal<Vec<Post>>, 
    filtered_ids: Signal<Option<HashSet<i32>>>,
) -> impl IntoView {
  let filtered_posts = move || 
    match filtered_ids.get() {
      Some(ids) => unfiltered_posts().into_iter().filter(|post| ids.contains(&post.post_id)).collect(),
      None => unfiltered_posts()
    };

  view! {
    <For each=filtered_posts key=|post| post.post_id let:post>
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
  }
}
