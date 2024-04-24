use chrono::NaiveDateTime;
use leptos::*;
use leptos_router::use_params;
use leptos_router::Params;
use serde::Deserialize;
use serde::Serialize;

use crate::util::global_state::GlobalState;

#[derive(Params, PartialEq, Clone)]
pub struct PostId {
    pub post_id: i32,
}

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Post {
    timestamp: NaiveDateTime,
    title: String,
    contents: String,
    author_name: String,
    anonymous: bool,
}

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Reply {
    time: NaiveDateTime,
    contents: String,
    author_name: String,
    anonymous: bool,
    replyid: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddReplyInfo {
    post_id: i32,
    contents: String,
    anonymous: bool,
}

#[component]
pub fn FocusedPost() -> impl IntoView {
    // Fetch post id from route in the format of "class/:class_id/:post_id"
    let post_id = use_params::<PostId>();
    let global_state = expect_context::<GlobalState>();

    let post_and_replies = create_resource(post_id, |post_id| async {
        get_post(post_id.unwrap().post_id).await.unwrap()
    });
    let post = move || post_and_replies().map(|tuple| tuple.0);
    let replies = move || post_and_replies().map(|tuple| tuple.1).unwrap_or_default();

    let (reply_contents, set_reply_contents) = create_signal(String::default());
    let (reply_anonymous_state, set_reply_anonymous_state) = create_signal(false);

    let add_reply_action = create_action(move |reply_info: &AddReplyInfo| {
        let reply_info = reply_info.clone();
        async move {
            match add_reply(reply_info, global_state.user_name.get().unwrap()).await {
                Ok(reply) => {
                    post_and_replies.update(|post_and_replies| {
                        post_and_replies
                            .iter_mut()
                            .for_each(|tuple| tuple.1.push(reply.clone()))
                    });
                    set_reply_contents(String::default());
                }
                Err(_) => logging::error!("Attempt to post reply failed. Please try again"),
            };
        }
    });

    view! {
        <div class="bg-white rounded shadow p-6 flex flex-col gap-3">
            <Suspense fallback=|| view! {
                <DarkenedCard class="h-32">"Loading..."</DarkenedCard>
            }>
                <DarkenedCard class="p-5">
                    <p class="font-bold text-lg">{move || post().map(|post| post.title)}</p>
                    <p class="font-light text-sm">
                        "Posted by "
                        {move || post().map(|post| post.author_name)}
                        {move || post().map(|post| format!("{}", post.timestamp.format(" at %l %p on %b %-d")))}
                    </p>
                    <br/>
                    <p>{move || post().map(|post| post.contents)}</p>
                    // TODO use the post's timestamp, author_name and anonymous info
                </DarkenedCard>
                <For
                each=replies
                key=|reply| reply.replyid
                let:reply
                >
                    <DarkenedCard class="p-5 ">
                        <p class="font-bold">
                            "Answered by "
                            {reply.author_name}
                            {format!("{}", reply.time.format(" at %l %p on %b %-d"))}
                            ":"
                        </p>
                        <br/>
                        <p>{reply.contents}</p>
                        // TODO use the reply's timestamp, author's name and anonymous info
                    </DarkenedCard>
                </For>
                <DarkenedCard class="p-5 flex flex-col gap-2">
                    <p>"Answer this post:"</p>
                    <div class="bg-white p-3 rounded-t-lg">
                        // Inner border
                        <div class="border border-gray-300 rounded-t-lg h-12"></div>
                        <textarea class="h-96 w-full resize-none border border-gray-300 rounded-b-lg p-2"
                            prop:value=reply_contents
                            on:input=move |ev| set_reply_contents(event_target_value(&ev))
                        >
                        </textarea>
                    </div>
                    <div class="flex justify-end gap-5">
                        <label
                        for="anonymousToggle"
                        class="flex items-center cursor-pointer select-none"
                        >
                            <span class="mx-2">"Anonymous:"</span>
                            <div class="relative">
                                <input
                                    type="checkbox"
                                    id="anonymousToggle"
                                    class="peer sr-only"
                                    prop:checked=reply_anonymous_state
                                    on:change=move |_| set_reply_anonymous_state(!reply_anonymous_state())
                                />
                                <div class="block h-8 rounded-full bg-gray-500 w-14"></div>
                                <div class="absolute w-6 h-6 transition bg-white rounded-full left-1 top-1 peer-checked:translate-x-full peer-checked:bg-primary"></div>
                            </div>
                        </label>
                        <button class="bg-gray-500 p-2 rounded-full text-white hover:bg-gray-600"
                            on:click=move |_| add_reply_action.dispatch(
                                AddReplyInfo {
                                    post_id: post_id().unwrap().post_id,
                                    contents: reply_contents(),
                                    anonymous: reply_anonymous_state()
                                })
                        >
                        "Post Response"
                        </button>
                    </div>
                </DarkenedCard>
            </Suspense>
        </div>
    }
}

#[component]
fn DarkenedCard(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!("bg-[#EEEEEE] rounded-xl {}", class)>{children()}</div>
    }
}

#[server(AddReply)]
pub async fn add_reply(reply_info: AddReplyInfo, user: String) -> Result<Reply, ServerFnError> {
    use crate::database_functions::UserId;
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to add Reply".to_string(),
    ))?;

    let user_id: UserId = sqlx::query_as("select id from users where name = $1")
        .bind(user)
        .fetch_one(&pool)
        .await
        .expect("select should work");

    let newreply: Reply = sqlx::query_as(
        "INSERT INTO replies (time, authorid, postid, anonymous, contents) 
                        VALUES (CURRENT_TIMESTAMP, $1, $2, $3, $4)
                RETURNING                 
                time, 
                contents,
                'You' as author_name, 
                anonymous,
                replyid;",
    )
    .bind(user_id.0)
    .bind(reply_info.post_id)
    .bind(reply_info.anonymous)
    .bind(reply_info.contents)
    .fetch_one(&pool)
    .await
    .map_err(|db_error| {
        logging::error!(
            "\nAdd Reply Server Function Failed. Database returned error {:?}\n",
            db_error
        );
        ServerFnError::<NoCustomError>::ServerError("Unable to add Reply".to_string())
    })?;

    Ok(newreply)
}

/**
 * Get all post information for a given the post id
 */
#[server(GetPost)]
pub async fn get_post(post_id: i32) -> Result<(Post, Vec<Reply>), ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;
    use tokio::*;

    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to fetch posts".to_string(),
    ))?;

    let (post, replies) = join!(
        sqlx::query_as::<_, Post>(
            "SELECT 
                timestamp,
                title, 
                contents, 
                CASE WHEN anonymous THEN 'Anonymous Author'
                    ELSE users.name 
                END as author_name, 
                anonymous 
            FROM posts JOIN users ON posts.authorid = users.id WHERE posts.postid = $1"
        )
        .bind(post_id)
        .fetch_optional(&pool),
        sqlx::query_as::<_, Reply>(
            "SELECT 
                time, 
                contents,
                CASE WHEN anonymous THEN 'Anonymous Author'
                    ELSE users.name 
                END as author_name, 
                anonymous,
                replyid
            FROM replies JOIN users ON replies.authorid = users.id WHERE replies.postid = $1"
        )
        .bind(post_id)
        .fetch_all(&pool)
    );
    Ok((post.unwrap().unwrap(), replies.unwrap()))
}
