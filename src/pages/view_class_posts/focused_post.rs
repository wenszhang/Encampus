/**
 * This file contains the FocusedPost component which is used to display a single post and its replies.
 */
use crate::data::database::class_functions::check_user_is_instructor;
use crate::data::database::post_functions::{remove_post, resolve_post, Post, PostFetcher};
use crate::data::database::reply_functions::{add_reply, approve_reply, remove_reply};
use crate::expect_logged_in_user;
use crate::pages::global_components::notification::{
    NotificationComponent, NotificationDetails, NotificationType,
};
use crate::pages::global_components::rich_text_box::{RichTextBox, TiptapContentWrapper};
use crate::pages::view_class_posts::class::ClassId;
use crate::resources::images::svgs::approval_icon::ApproveIcon;
use crate::resources::images::svgs::cancel_icon::CancelIcon;
use crate::resources::images::svgs::check_icon::CheckIcon;
use crate::resources::images::svgs::dots_icon::DotsIcon;
use crate::resources::images::svgs::edit_post_icon::EditPostIcon;
use crate::resources::images::svgs::remove_icon::RemoveIcon;
use crate::resources::images::svgs::unapprove_icon::UnapproveIcon;
use crate::resources::images::svgs::unresolved_icon::UnresolvedIcon;
use chrono::FixedOffset;
use chrono::NaiveDateTime;
use leptos::*;
use leptos_router::use_params;
use leptos_router::Params;
use leptos_router::ParamsError;
use serde::Deserialize;
use serde::Serialize;

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
    let (user, _) = expect_logged_in_user!();
    // Fetch post id from route in the format of "class/:class_id/:post_id"
    let post_id = move || {
        use_params::<PostId>()()
            .expect("Tried to render FocusedPost without a post_id in the route")
            .post_id
    };
    let class_id = move || {
        use_params::<ClassId>()()
            .expect("Tried to render FocusedPost without a class_id in the route")
            .class_id
    };

    let post_and_replies = create_resource(post_id, |post_id| async move {
        get_post_details(post_id)
            .await
            .expect("Failed to fetch post details")
    });

    let is_instructor = create_resource(class_id, move |class_id| {
        let user_id = user().id;
        async move {
            check_user_is_instructor(user_id, class_id)
                .await
                .unwrap_or(false)
        }
    });

    view! {
        <div class="flex flex-col gap-3 p-6 bg-white rounded shadow">
            <Suspense fallback=|| {
                view! { <DarkenedCard class="h-32">"Loading..."</DarkenedCard> }
            }>
                {move || {
                    post_and_replies()
                        .zip(is_instructor())
                        .map(|((post, replies), is_instructor)| {
                            let (replies, set_replies) = create_signal(replies);
                            view! {
                                <QuestionContent post=post class_id=class_id() is_instructor/>
                                <RepliesList
                                    replies
                                    is_instructor
                                    remove_reply_callback=move |reply_id_to_remove| {
                                        set_replies
                                            .update(|replies_vec| {
                                                let mut i = 0;
                                                while i < replies_vec.len() {
                                                    if replies_vec[i].reply_id == reply_id_to_remove {
                                                        replies_vec.remove(i);
                                                    } else {
                                                        i += 1;
                                                    }
                                                }
                                            })
                                    }
                                />

                                <CreateReply
                                    post_id=post_id()
                                    add_reply_callback=move |new_reply| {
                                        set_replies
                                            .update(move |replies_vec| replies_vec.push(new_reply))
                                    }
                                />
                            }
                        })
                }}

            </Suspense>
        </div>
    }.into_view()
}

#[component]
fn DarkenedCard(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("bg-[#EEEEEE] rounded-xl {}", class)>{children()}</div> }
}

#[component]
pub fn SelectOrderOption(
    value_and_label: &'static str,
    #[prop(default = false)] selected: bool,
) -> impl IntoView {
    view! {
        <option value=value_and_label selected=selected>
            {value_and_label}
        </option>
    }
}

#[component]
fn QuestionContent(post: PostDetails, class_id: i32, is_instructor: bool) -> impl IntoView {
    let (user, _) = expect_logged_in_user!();

    view! {
        <DarkenedCard class="relative p-5">
            <p class="text-lg font-bold">{&post.title}</p>
            <div class="flex gap-5 justify-end">
                <div class="flex items-center cursor-pointer select-none">
                    // Post Dropdown
                    {(post.author_id == user().id || is_instructor)
                        .then(move || {
                            view! {
                                <FocusedDropdown
                                    class_id
                                    post_id=post.post_id
                                    post_is_resolved=post.resolved
                                />
                            }
                        })}

                </div>
            </div>
            <p class="text-sm font-light">
                "Posted by " {post.author_first_name} " " {post.author_last_name} " "
                {post
                    .timestamp
                    .checked_add_offset(FixedOffset::west_opt(6 * 3600).unwrap())
                    .unwrap()
                    .format("at %l %p on %b %-d")
                    .to_string()}

            </p>
            <br/>
            <TiptapContentWrapper raw_html=post.contents/>
        // TODO use the post's timestamp
        </DarkenedCard>
    }
}

#[component]
fn CreateReply<F>(post_id: i32, add_reply_callback: F) -> impl IntoView
where
    F: Fn(Reply) + 'static + Copy,
{
    let (user, _) = expect_logged_in_user!();
    let (reply_contents, set_reply_contents) = create_signal(String::default());
    let (reply_anonymous_state, set_reply_anonymous_state) = create_signal(false);
    let class_id: Memo<Result<ClassId, leptos_router::ParamsError>> = use_params::<ClassId>();

    let (notification_details, set_notification_details) =
        create_signal(None::<NotificationDetails>);

    let (editor_count, set_editor_count) = create_signal(0);

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

    let add_reply_action = create_action(move |reply_info: &AddReplyInfo| {
        let reply_info = reply_info.clone();
        async move {
            match add_reply(reply_info, user().user_name).await {
                Ok(reply) => {
                    set_reply_contents(String::default());
                    set_editor_count.update(|x| *x += 1);
                    add_reply_callback(reply);
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

    view! {
        <DarkenedCard class="flex flex-col gap-2 p-5">
            <p>"Answer this post:"</p>
            <div class="p-3 h-96 bg-white rounded-t-lg">
                {move || {
                    view! {
                        <RichTextBox
                            id=format!("reply_rich_text_box_{}", editor_count())
                            set_value=set_reply_contents
                            value=reply_contents
                        />
                    }
                }}

            </div>
            <div class="flex gap-5 justify-end">
                <label for="anonymousToggle" class="flex items-center cursor-pointer select-none">
                    <span class="mx-2">"Reply Anonymously:"</span>
                    <div class="relative">
                        <input
                            type="checkbox"
                            id="anonymousToggle"
                            class="sr-only peer"
                            prop:checked=reply_anonymous_state
                            on:change=move |_| set_reply_anonymous_state(!reply_anonymous_state())
                        />
                        <div class="flex justify-evenly items-center w-16 h-8 text-xs bg-gray-500 rounded-full transition-colors peer-checked:bg-green-500">
                            <span class="[&:not(:peer-checked)]:invisible text-white">"On"</span>
                            <span class="peer-checked:invisible text-white">"Off"</span>
                        </div>
                        <div class="absolute top-1 left-1 w-6 h-6 bg-white rounded-full transition peer-checked:translate-x-8 peer-checked:bg-primary"></div>

                    </div>
                </label>
                <button
                    class="ml-4 py-2 px-4 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-red-500 hover:bg-red-600 focus:ring-offset-red-500 flex items-center gap-2"
                    type="button"
                    on:click=move |_| {
                    let navigate = leptos_router::use_navigate();
                    navigate(
                        format!("/classes/{}", class_id.get().unwrap().class_id).as_str(),
                        Default::default(),
                    );
                    }
                >
                <CancelIcon size="1em"/>
                    "Cancel"
                </button>
                <button
                    class="py-2 px-6 text-white rounded-full bg-customBlue hover:bg-customBlue-HOVER"
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
                                post_id,
                                contents: reply_contents(),
                                anonymous: reply_anonymous_state(),
                            })
                    }
                >

                    "Post Response +"
                </button>
                {notification_view}
            </div>
        </DarkenedCard>
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
pub fn FocusedDropdown(class_id: i32, post_id: i32, post_is_resolved: bool) -> impl IntoView {
    let (user, _) = expect_logged_in_user!();
    let (_notification_details, set_notification_details) =
        create_signal(None::<NotificationDetails>);
    let posts = expect_context::<Resource<PostFetcher, Vec<Post>>>();

    let remove_action = create_action(move |post_id: &PostId| {
        let post_id = post_id.post_id;
        async move {
            match get_post_details(post_id).await {
                Ok(current_post) => {
                    if (remove_post(post_id, user().id).await).is_ok() {
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
                    // Reload to show updated status
                    let window = web_sys::window().expect("should have a Window");
                    window.location().reload().expect("failed to reload page");
                }
            }
        }
    });

    let (menu_visible, set_menu_visible) = create_signal(false);

    let toggle_menu = { move |_| set_menu_visible(!menu_visible.get()) };

    view! {
        <div class="flex absolute top-1 right-2 z-20 items-center">
            <button on:click=toggle_menu class="rounded-lg bg-card-header hover:shadow-customInset">
                <DotsIcon size="36px"/>
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
                                <button
                                    class="inline-flex items-center p-1 w-full text-left leading-tight text-gray-700 rounded-md hover:text-black hover:bg-gray-100"
                                    on:click=move |_| {
                                        set_menu_visible(false);
                                        let navigate = leptos_router::use_navigate();
                                        navigate(
                                            format!("/classes/{}/{}/edit", class_id, post_id).as_str(),
                                            Default::default(),
                                        );
                                    }
                                >

                                    <EditPostIcon size="20px"/>
                                    <span class="ml-2">Edit</span>
                                </button>
                                {if post_is_resolved {
                                    view! {
                                        <button
                                            class="inline-flex items-center p-1 w-full text-left leading-tight text-customYellow-details rounded-md hover:bg-gray-100"
                                            on:click=move |_| {
                                                resolve_action.dispatch(PostId { post_id });
                                                set_menu_visible(false);
                                            }
                                        >

                                            <UnresolvedIcon size="20px"/>
                                            <span class="ml-2">Unresolve</span>
                                        </button>
                                    }
                                } else {
                                    view! {
                                        <button
                                            class="inline-flex items-center p-1 w-full text-sm leading-tight text-customGreen-details rounded-md hover:bg-gray-100"
                                            on:click=move |_| {
                                                resolve_action.dispatch(PostId { post_id });
                                                set_menu_visible(false);
                                            }
                                        >

                                            <CheckIcon size="20px"/>
                                            <span class="ml-2">Resolve</span>
                                        </button>
                                    }
                                }}

                                <button
                                    class="inline-flex items-center p-1 w-full text-sm leading-tight text-red-500 rounded-md hover:bg-gray-100"
                                    on:click=move |_| {
                                        remove_action.dispatch(PostId { post_id });
                                        set_menu_visible(false);
                                    }
                                >

                                    <RemoveIcon size="20px"/>
                                    <span class="ml-2">Remove</span>
                                </button>
                            </div>
                        }
                            .into_view()
                    }}

                </div>
            </div>
        </div>
    }.into_view()
}

type PostAndReplies = Resource<Result<PostId, ParamsError>, Option<(PostDetails, Vec<Reply>)>>;

#[component]
pub fn ReplyDropdown<F>(
    remove_reply_callback: F,
    reply_approved: bool,
    reply_id: i32,
    is_instructor: bool,
) -> impl IntoView
where
    F: Fn(i32) + 'static + Copy,
{
    let (user, _) = expect_logged_in_user!();
    let (_notification_details, set_notification_details) =
        create_signal(None::<NotificationDetails>);

    let remove_action = create_action(move |reply_id: &ReplyId| {
        let reply_id = reply_id.reply_id;
        async move {
            match remove_reply(reply_id, user().id).await {
                Ok(_) => {
                    remove_reply_callback(reply_id);
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
            let _ = approve_reply(reply_id, user().id, true).await;
        }
    });

    let unapprove_action = create_action(move |reply_id: &ReplyId| {
        let reply_id = reply_id.reply_id;
        async move {
            let _ = approve_reply(reply_id, user().id, false).await;
        }
    });

    let (menu_visible, set_menu_visible) = create_signal(false);
    let toggle_menu = { move |_| set_menu_visible(!menu_visible.get()) };

    view! {
        <div class="flex absolute top-0 right-2 z-20 items-center">
            <button on:click=toggle_menu class="rounded-lg bg-card-header hover:shadow-customInset">
                <DotsIcon size="36px"/>
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
                                    if reply_approved {
                                        view! {
                                            <div>
                                                <button
                                                    class="inline-flex items-center p-2 w-full text-sm leading-tight text-customYellow-details rounded-md hover:bg-gray-100"
                                                    on:click=move |_| {
                                                        unapprove_action.dispatch(ReplyId { reply_id });
                                                        set_menu_visible(false);
                                                    }
                                                >
                                                   <UnapproveIcon size="1em"/> 
                                                    <span class="ml-2">Unapprove</span>
                                                </button>
                                            </div>
                                        }
                                    } else {
                                        view! {
                                            <div>
                                                <button
                                                    class="inline-flex items-center p-2 w-full text-sm leading-tight text-customGreen-details rounded-md hover:text-black hover:bg-gray-100"
                                                    on:click=move |_| {
                                                        approve_action.dispatch(ReplyId { reply_id });
                                                        set_menu_visible(false);
                                                    }
                                                >
                                                    <ApproveIcon size="20px"/>
                                                    <span class="ml-2">Approve</span>
                                                </button>
                                            </div>
                                        }
                                    }
                                } else {
                                    view! { <div></div> }
                                }}
                                <button
                                class="inline-flex items-center p-2 w-full text-sm leading-tight text-red-500 rounded-md hover:text-red-500 hover:bg-gray-100"
                                    on:click=move |_| {
                                        remove_action.dispatch(ReplyId { reply_id });
                                        set_menu_visible(false);
                                    }
                                >
                                <RemoveIcon size="20px" />
                                    <span class="ml-2">Remove</span>
                                </button>

                            </div>
                        }
                            .into_view()
                    }}

                </div>
            </div>
        </div>
    }.into_view()
}

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

#[component]
fn RepliesList<F>(
    replies: ReadSignal<Vec<Reply>>,
    remove_reply_callback: F,
    is_instructor: bool,
) -> impl IntoView
where
    F: Fn(i32) + 'static + Copy,
{
    let (user, _) = expect_logged_in_user!();
    let (_order_option, set_order_option) = create_signal("Newest First".to_string());

    let order_option_memo = create_memo(move |_| _order_option());

    let sorted_replies = move || {
        let order = order_option_memo();
        sort_replies(replies(), &order)
            .into_iter()
            .filter(|reply| !reply.removed)
            .collect::<Vec<_>>()
    };

    view! {
        <div>
            {move || {
                if sorted_replies().is_empty() {
                    view! {
                        <span>
                            <b>"No Replies Yet"</b>
                        </span>
                    }
                        .into_view()
                } else {
                    view! {
                        <div class="flex justify-between">
                            <b class="inline-block">"Replies:"</b>
                            <span class="inline-block">
                                <select on:change=move |ev| {
                                    let new_value = event_target_value(&ev);
                                    set_order_option(new_value);
                                }>
                                    <SelectOrderOption
                                        selected=true
                                        value_and_label="Newest First"
                                    />
                                    <SelectOrderOption value_and_label="Oldest First"/>
                                // <SelectOrderOption value_and_label="By Rating"/>
                                </select>
                            </span>
                        </div>
                    }
                        .into_view()
                }
            }}

        </div>
        <For each=sorted_replies key=|reply| reply.reply_id let:reply>
            <div>
                <DarkenedCard class="relative p-5">
                    <p class="font-bold">
                        "Answered by " {reply.author_name}
                        {reply
                            .time
                            .checked_add_offset(FixedOffset::west_opt(6 * 3600).unwrap())
                            .unwrap()
                            .format(" at %l %p on %b %-d")
                            .to_string()} ":"
                    </p>
                    <div class="flex gap-5 justify-end">
                        <div class="flex items-center cursor-pointer select-none">
                            {(reply.author_id == user().id || is_instructor)
                                .then(move || {
                                    view! {
                                        <div>
                                            <ReplyDropdown
                                                remove_reply_callback
                                                reply_id=reply.reply_id
                                                reply_approved=reply.approved
                                                is_instructor=is_instructor
                                            />
                                        </div>
                                    }
                                })}

                        </div>
                    </div>
                    <br/>
                    <TiptapContentWrapper raw_html=reply.contents/>
                    {reply
                        .approved
                        .then_some(
                            view! {
                                <p class="text-sm font-light">"Instructor Approved Response"</p>
                            },
                        )}

                // TODO use the reply's timestamp, author's name and anonymous info
                </DarkenedCard>
            </div>
        </For>
    }
    .into_view()
}
