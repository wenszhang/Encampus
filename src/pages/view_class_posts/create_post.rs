use super::class::ClassId;
/**
 * This file contains the CreatePost component, which is a form that allows users to create a new post.
 */
use crate::data::database::post_functions::{add_post, Post, PostFetcher};
use leptos::*;
use leptos_router::use_params;
use serde::{Deserialize, Serialize};

use crate::data::global_state::GlobalState;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddPostInfo {
    pub title: String,
    pub contents: String,
    pub anonymous: bool,
    pub limited_visibility: bool,
    pub classid: i32,
    pub private: bool,
}

#[component]
pub fn CreatePost() -> impl IntoView {
    let class_id = use_params::<ClassId>();
    let global_state = expect_context::<GlobalState>();
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

    let add_post_action = create_action(move |postInfo: &AddPostInfo| {
        let postInfo = postInfo.clone();
        async move {
            match add_post(postInfo, global_state.id.get_untracked().unwrap()).await {
                Ok(post) => {
                    let post_id = post.post_id;
                    posts.update(|posts| {
                        if let Some(posts) = posts {
                            posts.push(post);
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
        <DarkenedCard class="p-5 flex flex-col gap-2">
            <p>"Create New Post"</p>
            <div class="bg-white p-3 rounded-t-lg">
                // Inner border
                <p>"Title:"</p>
                <textarea class="h-12 w-full resize-none border border-gray-300 rounded-t-lg p-2"
                    on:input=on_input(set_post_title)
                    prop:value=post_title
                >
                </textarea>
                //<div class="border border-gray-300 rounded-t-lg h-12"></div>
                <p>"Contents:"</p>
                <textarea class="h-96 w-full resize-none border border-gray-300 rounded-b-lg p-2"
                    on:input=on_input(set_post_contents)
                    prop:value=post_contents
                >
                </textarea>
            </div>
            <div class="flex justify-end gap-5">
                <label
                for="privateToggle"
                class="flex items-center cursor-pointer select-none"
                >
                    <span class="mx-2">"Private:"</span>
                    <div class="relative">
                        <input
                            type="checkbox"
                            id="privateToggle"
                            class="peer sr-only"
                            prop:checked=private_state
                            on:change=move |_| set_private_state(!private_state())
                        />
                        <div class="block h-8 rounded-full bg-gray-500 w-14"></div>
                        <div class="absolute w-6 h-6 transition bg-white rounded-full left-1 top-1 peer-checked:translate-x-full peer-checked:bg-primary"></div>
                    </div>
                </label>
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
                            prop:checked=anonymous_state
                            on:change=move |_| set_anonymous_state(!anonymous_state())
                        />
                        <div class="block h-8 rounded-full bg-gray-500 w-14"></div>
                        <div class="absolute w-6 h-6 transition bg-white rounded-full left-1 top-1 peer-checked:translate-x-full peer-checked:bg-primary"></div>
                    </div>
                </label>
                <button type="submit" class="bg-gray-500 p-2 rounded-full text-white hover:bg-gray-600"
                    on:click=move |_| {
                        if post_title().is_empty() || post_contents().is_empty() {
                            return; // Probably want to write an error message on the screen, might add that sooner if not later
                        }
                        add_post_action.dispatch(
                            AddPostInfo {
                                title: post_title(),
                                contents: post_contents(),
                                anonymous: anonymous_state(),
                                limited_visibility: false,
                                classid: class_id.get().unwrap().class_id,
                                private: private_state(),
                            })
                    }
                >
                "Post"
                </button>
            </div>
        </DarkenedCard>
    }
}

#[component]
fn DarkenedCard(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!("bg-[#EEEEEE] rounded-xl {}", class)>{children()}</div>
    }
}
