use crate::data::database::class_functions::check_user_is_instructor;
/**
 * This file contains the FocusedPost component which is used to display a single post and its replies.
 */
use crate::resources::images::svgs::text_area_icon::TextAreaIcon;
use chrono::FixedOffset;
use chrono::NaiveDateTime;
use leptos::*;
use leptos_router::use_params;
use leptos_router::Params;
use leptos_router::ParamsError;
use serde::Deserialize;
use serde::Serialize;

use crate::data::database::class_functions::get_instructor;
use crate::data::database::post_functions::{remove_post, resolve_post, Post, PostFetcher};
use crate::data::database::reply_functions::{add_reply, approve_reply, remove_reply};
use crate::data::global_state::GlobalState;
use crate::pages::global_components::notification::{
    NotificationComponent, NotificationDetails, NotificationType,
};
use crate::pages::view_class_posts::class::ClassId;
use crate::resources::images::svgs::dots_icon::DotsIcon;

#[derive(Params, PartialEq, Clone)]
pub struct PostId {
    pub post_id: i32,
}

#[derive(Params, PartialEq, Clone)]
pub struct ReplyId {
    pub reply_id: i32,
}

/**
 * Struct that holds post details
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct PostDetails {
    pub post_id: i32,
    pub timestamp: NaiveDateTime,
    pub title: String,
    pub contents: String,
    pub author_first_name: String,
    pub author_last_name: String,
    pub anonymous: bool,
    pub resolved: bool,
    pub author_id: i32,
    pub private: bool,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Reply {
    pub time: NaiveDateTime,
    pub contents: String,
    pub author_name: String,
    pub author_id: i32,
    pub anonymous: bool,
    pub reply_id: i32,
    pub removed: bool,
    pub approved: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddReplyInfo {
    pub post_id: i32,
    pub contents: String,
    pub anonymous: bool,
}

#[component]
pub fn FocusedPost() -> impl IntoView {
    let post_id = use_params::<PostId>();
    let class_id = use_params::<ClassId>();
    let global_state = expect_context::<GlobalState>();
    let (order_option, set_value) = create_signal("Newest First".to_string());
    let (notification_details, set_notification_details) =
        create_signal(None::<NotificationDetails>);
    let posts = expect_context::<Resource<PostFetcher, Vec<Post>>>();

    let post_and_replies = create_resource(post_id, |post_id| async {
        if let Ok(post_id) = post_id {
            Some(get_post_details(post_id.post_id).await.unwrap())
        } else {
            None
        }
    });

    let post = move || post_and_replies().flatten().map(|tuple| tuple.0);
    let replies = move || {
        post_and_replies()
            .flatten()
            .map(|tuple| tuple.1)
            .unwrap_or_default()
    };

    let (reply_contents, set_reply_contents) = create_signal(String::default());
    let (reply_anonymous_state, set_reply_anonymous_state) = create_signal(false);

    let add_reply_action = create_action(move |reply_info: &AddReplyInfo| {
        let reply_info = reply_info.clone();
        async move {
            match add_reply(reply_info, global_state.user_name.get_untracked().unwrap()).await {
                Ok(reply) => {
                    post_and_replies.update(|post_and_replies| {
                        if let Some(outer_option) = post_and_replies.as_mut() {
                            if let Some(post_and_replies) = outer_option.as_mut() {
                                post_and_replies.1.push(reply.clone())
                            }
                        }
                    });
                    set_reply_contents(String::default());
                }
                Err(_) => {
                    logging::error!("Attempt to post reply failed. Please try again");
                    set_notification_details(Some(NotificationDetails {
                        message: "Failed to add reply. Please try again.".to_string(),
                        notification_type: NotificationType::Error,
                    }));
                }
            };
        }
    });

    fn sort_replies(replies: Vec<Reply>, order: &str) -> Vec<Reply> {
        let mut sorted_replies = replies.clone();
        match order {
            "Newest First" => sorted_replies.sort_by(|a, b| b.time.cmp(&a.time)),
            "Oldest First" => sorted_replies.sort_by(|a, b| a.time.cmp(&b.time)),
            // Add more sorting options here
            _ => (),
        }
        sorted_replies
    }

    let sorted_replies = create_memo(move |_| {
        let order = order_option.get();
        let replies = replies();
        sort_replies(replies, &order)
    });

    let instructor = create_resource(post_id, |post_id| async {
        get_instructor(post_id.unwrap().post_id)
            .await
            .unwrap_or_else(|_| "Failed".to_string())
    });

    let is_instructor = create_resource(class_id, move |class_id| {
        let user_id = global_state.id.get_untracked().unwrap_or_default();
        async move {
            check_user_is_instructor(user_id, class_id.unwrap().class_id)
                .await
                .unwrap_or(false)
        }
    });

    let notification_view = move || {
        notification_details.get().map(|details| {
            view! {
              <NotificationComponent
                notification_details=details.clone()
                on_close=move || set_notification_details(None)
              />
            }
        })
    };

    view! {
      <div class="flex flex-col gap-3 p-6 bg-white rounded shadow">
        <Suspense fallback=|| view! { <DarkenedCard class="h-32">"Loading..."</DarkenedCard> }>
          <DarkenedCard class="relative p-5">
            <p class="text-lg font-bold">{move || post().map(|post| post.title)}</p>

            <div class="flex gap-5 justify-end">
              <div class="flex items-center cursor-pointer select-none">
                // Post Dropdown
                {post()
                  .map(|post| post.author_id)
                  .filter(|&author_id| author_id == global_state.id.get().unwrap_or_default())
                  .or_else(|| {
                    if is_instructor().unwrap_or_default() {
                      Some(class_id.get().unwrap().class_id)
                    } else {
                      None
                    }
                  })
                  .map(|_| {
                    view! {
                      <FocusedDropdown
                        class_id=class_id.get().unwrap().class_id
                        posts=posts
                        post=post().unwrap()
                      />
                    }
                  })}
              </div>
            </div>

            <p class="text-sm font-light">
              "Posted by " {move || post().map(|post| post.author_first_name)} " "
              {move || post().map(|post| post.author_last_name)}
              {move || {
                post()
                  .map(|post| {
                    format!(
                      "{}",
                      post
                        .timestamp
                        .checked_add_offset(FixedOffset::west_opt(6 * 3600).unwrap())
                        .unwrap()
                        .format(" at %l %p on %b %-d"),
                    )
                  })
              }}
            </p>
            <br />
            <p>{move || post().map(|post| post.contents)}</p>
          // TODO use the post's timestamp
          </DarkenedCard>
          <div>
            {move || {
              if replies().is_empty() {
                view! {
                  <span>
                    <b>"No Replies Yet"</b>
                  </span>
                }
              } else {
                view! {
                  <span class="inline-block flex justify-between">
                    <b class="inline-block">"Replies:"</b>
                    <span class="inline-block">
                      <select on:change=move |ev| {
                        let new_value = event_target_value(&ev);
                        set_value(new_value);
                      }>
                        <SelectOrderOption order_option is="Newest First" />
                        <SelectOrderOption order_option is="Oldest First" />
                      // <SelectOrderOption order_option is="By Rating"/>
                      </select>
                    </span>
                  </span>
                }
              }
            }}
          </div>
          <For each=sorted_replies key=|reply| reply.reply_id let:reply>
            {if !reply.removed {
              view! {
                <div>
                  <DarkenedCard class="relative p-5">
                    <p class="font-bold">
                      "Answered by " {reply.clone().author_name}
                      {format!(
                        "{}",
                        reply
                          .time
                          .checked_add_offset(FixedOffset::west_opt(6 * 3600).unwrap())
                          .unwrap()
                          .format(" at %l %p on %b %-d"),
                      )} ":"
                    </p>
                    <div class="flex gap-5 justify-end">
                      <div class="flex items-center cursor-pointer select-none">
                        {if reply.author_id == global_state.id.get().unwrap_or_default()
                          || is_instructor().unwrap_or_default()
                        {
                          let reply = reply.clone();
                          view! {
                            <div>
                              <ReplyDropdown
                                post_and_replies=post_and_replies
                                reply=reply.clone()
                                is_instructor=is_instructor().unwrap_or_default()
                              />
                            </div>
                          }
                        } else {
                          view! { <div></div> }
                        }}
                      </div>
                    </div>
                    <br />
                    <p>{reply.contents}</p>
                    {if reply.approved {
                      view! { <p class="text-sm font-light">"Instructor Approved Response"</p> }
                    } else {
                      view! { <p class="text-sm font-light"></p> }
                    }}
                  // TODO use the reply's timestamp, author's name and anonymous info
                  </DarkenedCard>
                </div>
              }
            } else {
              view! { <div></div> }
            }}
          </For>
          <DarkenedCard class="flex flex-col gap-2 p-5">
            <p>"Answer this post:"</p>
            <div class="p-3 bg-white rounded-t-lg">
              // Inner border
              <div class="flex items-center h-12 rounded-t-lg border border-gray-300">
                <TextAreaIcon />
              </div>
              <textarea
                class="p-2 w-full h-96 rounded-b-lg border border-gray-300 resize-none"
                prop:value=reply_contents
                on:input=move |ev| set_reply_contents(event_target_value(&ev))
              ></textarea>
            </div>
            <div class="flex gap-5 justify-end">
              <label for="anonymousToggle" class="flex items-center cursor-pointer select-none">
                <span class="mx-2">"Anonymous:"</span>
                <div class="relative">
                  <input
                    type="checkbox"
                    id="anonymousToggle"
                    class="sr-only peer"
                    prop:checked=reply_anonymous_state
                    on:change=move |_| set_reply_anonymous_state(!reply_anonymous_state())
                  />
                  <div class="flex justify-evenly items-center w-16 h-8 text-xs bg-gray-500 rounded-full transition-colors peer-checked:bg-green-500">
                    <span class="[&:not(:peer-checked)]:invisible">"On"</span>
                    <span class="peer-checked:invisible">"Off"</span>
                  </div>
                  <div class="absolute top-1 left-1 w-6 h-6 bg-white rounded-full transition peer-checked:translate-x-8 peer-checked:bg-primary"></div>

                </div>
              </label>
              <button
                class="p-2 text-white rounded-full bg-customBlue hover:bg-customBlue-HOVER"
                on:click=move |_| {
                  if reply_contents().is_empty() {
                    set_notification_details(
                      Some(NotificationDetails {
                        message: "Response cannot be empty.".to_string(),
                        notification_type: NotificationType::Warning,
                      }),
                    );
                    return;
                  }
                  add_reply_action
                    .dispatch(AddReplyInfo {
                      post_id: post_id().unwrap().post_id,
                      contents: reply_contents(),
                      anonymous: reply_anonymous_state(),
                    })
                }
              >
                "Post Response"
              </button>
              {notification_view}
            </div>
          </DarkenedCard>
          <div class="flex gap-5 justify-end">
            <div class="flex items-center cursor-pointer select-none">
              // For some crazy reason removing this makes the dropdown up above not show up
              // It's totally not related but no clue why
              {if post().map(|post| post.author_id)
                == Some(global_state.id.get().unwrap_or_default())
                || instructor() == Some(global_state.user_name.get().unwrap_or_default())
              {}}
            </div>
          </div>
        </Suspense>
      </div>
    }
}

#[component]
fn DarkenedCard(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("bg-[#EEEEEE] rounded-xl {}", class)>{children()}</div> }
}

#[component]
pub fn SelectOrderOption(is: &'static str, order_option: ReadSignal<String>) -> impl IntoView {
    view! {
      <option order_option=is selected=move || order_option() == is>
        {is}
      </option>
    }
}

/**
 * Get all post information for a given the post id
 */
#[server(GetPostDetails)]
pub async fn get_post_details(post_id: i32) -> Result<(PostDetails, Vec<Reply>), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;
    use tokio::*;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to fetch posts".to_string(),
    ))?;

    let (post, replies) = join!(
        sqlx::query_as::<_, PostDetails>(
            "SELECT 
                postid as post_id,
                timestamp,
                title, 
                contents, 
                CASE WHEN anonymous THEN 'Anonymous'
                    ELSE users.firstname 
                END as author_first_name, 
                CASE WHEN anonymous THEN 'Author'
                    ELSE users.lastname
                END as author_last_name,
                anonymous,
                resolved, 
                authorid as author_id,
                private
            FROM posts JOIN users ON posts.authorid = users.id WHERE posts.postid = $1"
        )
        .bind(post_id)
        .fetch_optional(&pool),
        sqlx::query_as::<_, Reply>(
            "SELECT 
                time, 
                contents,
                CASE WHEN anonymous THEN 'Anonymous Author'
                    ELSE users.firstname 
                END as author_name, 
                authorid as author_id,
                anonymous,
                replyid as reply_id,
                removed,
                approved
            FROM replies JOIN users ON replies.authorid = users.id WHERE replies.postid = $1
            ORDER BY time;"
        )
        .bind(post_id)
        .fetch_all(&pool)
    );
    Ok((post.unwrap().unwrap(), replies.unwrap()))
}

#[component]
pub fn FocusedDropdown(
    class_id: i32,
    posts: Resource<PostFetcher, Vec<Post>>,
    post: PostDetails,
) -> impl IntoView {
    let global_state: GlobalState = expect_context::<GlobalState>();
    let (_notification_details, set_notification_details) =
        create_signal(None::<NotificationDetails>);

    let remove_action = create_action(move |post_id: &PostId| {
        let post_id = post_id.post_id;
        async move {
            match get_post_details(post_id).await {
                Ok(current_post) => {
                    if let Ok(()) =
                        remove_post(post_id, global_state.id.get_untracked().unwrap()).await
                    {
                        posts.update(|posts| {
                            if let Some(index) = posts
                                .as_mut()
                                .unwrap()
                                .iter()
                                .position(|post| post.post_id == current_post.0.post_id)
                            {
                                posts.as_mut().unwrap().remove(index);
                            }

                            let navigate = leptos_router::use_navigate();
                            navigate(
                                format!("/classes/{}", class_id,).as_str(),
                                Default::default(),
                            );
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

    let resolve_action = create_action(move |post_id: &PostId| {
        let post_id = post_id.post_id;
        async move {
            if let Ok(current_post) = get_post_details(post_id).await {
                if (resolve_post(post_id, !current_post.0.resolved).await).is_ok() {
                    posts.update(|posts| {
                        if let Some(posts) = posts.as_mut() {
                            if let Some(index) = posts
                                .iter()
                                .position(|post| post.post_id == current_post.0.post_id)
                            {
                                if let Some(post) = posts.get_mut(index) {
                                    post.resolved = !post.resolved;
                                }
                            }
                        }
                    });
                }
            }
        }
    });

    let (menu_visible, set_menu_visible) = create_signal(false);

    let toggle_menu = { move |_| set_menu_visible(!menu_visible.get()) };

    view! {
      <div class="flex absolute top-0 right-2 z-20 items-center">
        <button on:click=toggle_menu class="rounded-lg bg-card-header hover:shadow-customInset">
          <DotsIcon size="36px" />
        </button>
        <div class=move || {
          if menu_visible.get() {
            "absolute right-0 top-0 mt-7 w-30 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5"
          } else {
            "hidden"
          }
        }>

          <div class="pr-2 text-right">
            {move || {
              view! {
                <div class="p-3 rounded-md w-30">
                  {if post.resolved {
                    view! {
                      <button
                        class="inline-flex items-center p-1 w-full text-left text-gray-700 rounded-md hover:text-black hover:bg-gray-100"
                        on:click=move |_| {
                          resolve_action.dispatch(PostId { post_id: post.post_id });
                          set_menu_visible(false);
                        }
                      >
                        <span class="ml-2">Unresolve</span>
                      </button>
                    }
                  } else {
                    view! {
                      <button
                        class="inline-flex items-center p-1 w-full text-left text-gray-700 rounded-md hover:text-black hover:bg-gray-100"
                        on:click=move |_| {
                          resolve_action.dispatch(PostId { post_id: post.post_id });
                          set_menu_visible(false);
                        }
                      >
                        <span class="ml-2">Resolve</span>
                      </button>
                    }
                  }}
                  <button
                    class="inline-flex items-center p-1 w-full text-left text-gray-700 rounded-md hover:text-black hover:bg-gray-100"
                    on:click=move |_| {
                      remove_action.dispatch(PostId { post_id: post.post_id });
                      set_menu_visible(false);
                    }
                  >
                    <span class="ml-2">Remove</span>
                  </button>
                </div>
              }
                .into_view()
            }}
          </div>
        </div>
      </div>
    }
}

type PostAndReplies = Resource<Result<PostId, ParamsError>, Option<(PostDetails, Vec<Reply>)>>;

#[component]
pub fn ReplyDropdown(
    post_and_replies: PostAndReplies,
    reply: Reply,
    is_instructor: bool,
) -> impl IntoView {
    let global_state: GlobalState = expect_context::<GlobalState>();
    let (_notification_details, set_notification_details) =
        create_signal(None::<NotificationDetails>);

    let remove_action = create_action(move |reply_id: &ReplyId| {
        let reply_id = reply_id.reply_id;
        async move {
            match remove_reply(reply_id, global_state.id.get_untracked().unwrap()).await {
                Ok(_) => {
                    post_and_replies.update(|post_and_replies| {
                        if let Some(outer_option) = post_and_replies.as_mut() {
                            if let Some(post_and_replies) = outer_option.as_mut() {
                                if let Some(index) = post_and_replies
                                    .1
                                    .iter()
                                    .position(|r| r.reply_id == reply.reply_id)
                                {
                                    post_and_replies.1.remove(index);
                                }
                            }
                        }
                    });
                }
                Err(_) => {
                    logging::error!("Attempt to remove reply failed. Please try again");
                    set_notification_details(Some(NotificationDetails {
                        message: "Failed to remove reply. Please try again.".to_string(),
                        notification_type: NotificationType::Error,
                    }));
                }
            }
        }
    });

    let approve_action = create_action(move |reply_id: &ReplyId| {
        let reply_id = reply_id.reply_id;
        async move {
            let _ = approve_reply(reply_id, global_state.id.get_untracked().unwrap(), true).await;
        }
    });

    let unapprove_action = create_action(move |reply_id: &ReplyId| {
        let reply_id = reply_id.reply_id;
        async move {
            let _ = approve_reply(reply_id, global_state.id.get_untracked().unwrap(), false).await;
        }
    });

    let (menu_visible, set_menu_visible) = create_signal(false);
    let toggle_menu = { move |_| set_menu_visible(!menu_visible.get()) };

    view! {
      <div class="flex absolute top-0 right-2 z-20 items-center">
        <button on:click=toggle_menu class="rounded-lg bg-card-header hover:shadow-customInset">
          <DotsIcon size="36px" />
        </button>
        <div class=move || {
          if menu_visible.get() {
            "absolute right-0 top-0 mt-7 w-30 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5"
          } else {
            "hidden"
          }
        }>

          <div class="pr-2 text-right">
            {move || {
              view! {
                <div class="p-3 rounded-md w-30">
                  {if is_instructor {
                    if reply.approved {

                      view! {
                        <div>
                          <button
                            class="inline-flex items-center p-1 w-full text-left text-gray-700 rounded-md hover:text-black hover:bg-gray-100"
                            on:click=move |_| {
                              unapprove_action
                                .dispatch(ReplyId {
                                  reply_id: reply.reply_id,
                                });
                              set_menu_visible(false);
                            }
                          >
                            <span class="ml-2">Unapprove</span>
                          </button>
                        </div>
                      }
                    } else {
                      view! {
                        <div>
                          <button
                            class="inline-flex items-center p-1 w-full text-left text-gray-700 rounded-md hover:text-black hover:bg-gray-100"
                            on:click=move |_| {
                              approve_action
                                .dispatch(ReplyId {
                                  reply_id: reply.reply_id,
                                });
                              set_menu_visible(false);
                            }
                          >
                            <span class="ml-2">Approve</span>
                          </button>
                        </div>
                      }
                    }
                  } else {
                    view! { <div></div> }
                  }}
                  <button
                    class="inline-flex items-center p-1 w-full text-left text-gray-700 rounded-md hover:text-black hover:bg-gray-100"
                    on:click=move |_| {
                      remove_action
                        .dispatch(ReplyId {
                          reply_id: reply.reply_id,
                        });
                      set_menu_visible(false);
                    }
                  >
                    <span class="ml-2">Remove</span>
                  </button>
                </div>
              }
                .into_view()
            }}
          </div>
        </div>
      </div>
    }
}
