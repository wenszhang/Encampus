/**
 * This file contains the CreatePost component, which is a form that allows
 * users to create a new post.
 */
use super::class::ClassId;
use crate::resources::images::svgs::create_post_icon::CreatePostIcon;

use crate::{
    data::database::post_functions::{add_post, Post, PostFetcher},
    expect_logged_in_user,
    pages::global_components::rich_text_box::RichTextBox,
};
use leptoaster::*;
use leptos::*;
use leptos_router::use_params;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddPostInfo {
    pub title: String,
    pub contents: String,
    pub anonymous: bool,
    pub limited_visibility: bool,
    pub classid: i32,
    pub private: bool,
    pub ai_response: bool,
}

#[component]
pub fn CreatePost(on_new_post: impl Fn() + 'static) -> impl IntoView {
    let (user, _) = expect_logged_in_user!();
    let user_id = user().id;
    let class_id = use_params::<ClassId>();
    let posts = expect_context::<Resource<PostFetcher, Vec<Post>>>();

    let on_input = |setter: WriteSignal<String>| {
        move |ev| {
            setter(event_target_value(&ev));
        }
    };

    let (anonymous_state, set_anonymous_state) = create_signal(false);
    let (private_state, set_private_state) = create_signal(false);
    let (post_title, set_post_title) = create_signal("".to_string());
    let (post_contents, set_post_contents) = create_signal("".to_string());
    let (ai_response, set_ai_response) = create_signal(false);
    let toaster = expect_toaster(); // post submission confirmation.

    let add_post_action = create_action(move |postInfo: &AddPostInfo| {
        let postInfo = postInfo.clone();
        async move {
            match add_post(postInfo, user_id).await {
                Ok(post) => {
                    let post_id = post.post_id;
                    posts.update(|posts| {
                        if let Some(posts) = posts {
                            posts.insert(0, post);
                        }
                    });
                    let navigate = leptos_router::use_navigate();
                    navigate(
                        format!(
                            "/classes/{}/{}",
                            class_id.get_untracked().unwrap().class_id,
                            post_id
                        )
                        .as_str(),
                        Default::default(),
                    );
                }
                Err(_) => logging::error!("Attempt to post post failed. Please try again"),
            }
        }
    });

    view! {
        <DarkenedCard class="flex flex-col gap-2 p-5">
            <div class="flex items-center gap-2 px-3">
                <CreatePostIcon size="1.3em"/>
                <p class="pt-1">"Create New Post"</p>
            </div>
            <div class="p-3 bg-white rounded-t-lg">
                // Inner border
                <p>"Title:"</p>
                <div class="p-2">
                    <textarea
                        class="p-2 w-full h-12 rounded-lg border border-slate-400 resize-none"
                        on:input=on_input(set_post_title)
                        prop:value=post_title
                    ></textarea>
                </div>
                <p>"Contents:"</p>
                <div class="p-2 h-96">
                    <RichTextBox
                        id="create_post_rich_text_box".to_string()
                        set_value=set_post_contents
                        value=post_contents
                    />
                </div>
            </div>
            // AI assistant
            <div class="flex gap-5 justify-end">
                <label for="assistantToggle" class="flex items-center cursor-pointer select-none">
                    <span class="mx-2">"Encampus Assistant:"</span>
                    <div class="relative">
                        <input
                            type="checkbox"
                            id="assistantToggle"
                            class="sr-only peer"
                            prop:checked=ai_response
                            on:change=move |_| set_ai_response(!ai_response())
                        />
                        <div class="block w-14 h-8 bg-gray-500 rounded-full"></div>
                        <div class="absolute top-1 left-1 w-6 h-6 bg-white rounded-full transition peer-checked:translate-x-full peer-checked:bg-primary"></div>
                    </div>
                </label>
                // Private
                <label for="privateToggle" class="flex items-center cursor-pointer select-none">
                    <span class="mx-2">"Private to Instructors:"</span>
                    <div class="relative">
                        <input
                            type="checkbox"
                            id="privateToggle"
                            class="sr-only peer"
                            prop:checked=private_state
                            on:change=move |_| set_private_state(!private_state())
                        />
                        <div class="flex justify-evenly items-center w-14 h-8 text-xs bg-gray-500 rounded-full transition-colors peer-checked:bg-purple-500">
                            <span class="[&:not(:peer-checked)]:invisible text-white">"On"</span>
                            <span class="peer-checked:invisible text-white">"Off"</span>
                        </div>
                        <div class="absolute top-1 left-1 w-6 h-6 bg-white rounded-full transition peer-checked:translate-x-full peer-checked:bg-primary"></div>
                    </div>
                </label>
                // Anonymous
                <label for="anonymousToggle" class="flex items-center cursor-pointer select-none">
                    <span class="mx-2">"Post Anonymously:"</span>
                    <div class="relative">
                        <input
                            type="checkbox"
                            id="anonymousToggle"
                            class="sr-only peer"
                            prop:checked=anonymous_state
                            on:change=move |_| set_anonymous_state(!anonymous_state())
                        />
                        <div class="flex justify-evenly items-center w-14 h-8 text-xs bg-gray-500 rounded-full transition-colors peer-checked:bg-green-500">
                            <span class="[&:not(:peer-checked)]:invisible text-white">"On"</span>
                            <span class="peer-checked:invisible text-white">"Off"</span>
                        </div>
                        <div class="absolute top-1 left-1 w-6 h-6 bg-white rounded-full transition peer-checked:translate-x-full peer-checked:bg-primary"></div>
                    </div>
                </label>
                <button
                    type="submit"
                    class="py-2 px-4 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-customBlue hover:bg-customBlue-HOVER focus:ring-offset-customBlue"
                    on:click=move |_| {
                        if post_title().is_empty() || post_contents().is_empty() {
                            return;
                        }
                        add_post_action
                            .dispatch(AddPostInfo {
                                title: post_title(),
                                contents: post_contents(),
                                anonymous: anonymous_state(),
                                limited_visibility: false,
                                classid: class_id.get().unwrap().class_id,
                                private: private_state(),
                                ai_response: ai_response(),
                            });
                            // post confirmation.
                            toaster.toast(
                                ToastBuilder::new("Post created successfully!")
                                    .with_level(ToastLevel::Success)
                                    .with_dismissable(false)
                                    .with_expiry(Some(3_000))
                                    .with_progress(false)
                                    .with_position(ToastPosition::BottomRight)
                                );
                        on_new_post();
                    }
                >
                    "Post +"
                </button>
            </div>
        </DarkenedCard>
    }
}

#[component]
fn DarkenedCard(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("bg-[#EEEEEE] rounded-xl {}", class)>{children()}</div> }
}
