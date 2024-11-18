use super::class::ClassId;
use crate::pages::view_class_posts::class::IS_DISPLAYED_EDIT;
use crate::{
    data::database::post_functions::edit_post,
    expect_logged_in_user,
    pages::view_class_posts::focused_post::{get_post_details, PostId},
};
use leptos::*;
use leptos_router::use_params;

#[component]
pub fn EditPost() -> impl IntoView {
    let (user, _) = expect_logged_in_user!();
    let post_id = use_params::<PostId>();
    let class_id = use_params::<ClassId>();

    let post_and_replies = create_resource(post_id, |post_id| async {
        if let Ok(post_id) = post_id {
            Some(get_post_details(post_id.post_id).await.unwrap())
        } else {
            None
        }
    });

    let post = move || post_and_replies().flatten().map(|tuple| tuple.0);

    let (post_title, set_post_title) = create_signal(
        post()
            .as_ref()
            .map_or_else(|| "".to_string(), |p| p.title.clone()),
    );
    let (post_contents, set_post_contents) = create_signal(
        post()
            .as_ref()
            .map_or_else(|| "".to_string(), |p| p.contents.clone()),
    );
    let (private_state, set_private_state) =
        create_signal(post().as_ref().map_or_else(|| false, |p| p.private));
    let (anonymous_state, set_anonymous_state) =
        create_signal(post().as_ref().map_or_else(|| false, |p| p.anonymous));

    let edit_post_action = create_action(move |_| {
        let post_id = post_id.get().unwrap().post_id;
        // let post_info = AddPostInfo {
        //     title: post_title(),
        //     contents: post_contents(),
        //     anonymous: anonymous_state(),
        //     limited_visibility: false,
        //     classid: class_id,
        //     private: private_state(),
        //     ai_response: false,
        // };
        async move {
            match edit_post(post_id, post_title.get(), post_contents.get(), user().id).await {
                Ok(_) => {
                    let navigate = leptos_router::use_navigate();
                    navigate(
                        format!("/classes/{}", class_id.get().unwrap().class_id).as_str(),
                        Default::default(),
                    );
                }
                Err(e) => {
                    logging::error!("Error editing post: {:?}", e);
                }
            }
        }
    });

    let on_input = |setter: WriteSignal<String>| {
        move |ev| {
            setter(event_target_value(&ev));
        }
    };

    view! {
      <DarkenedCard class="flex flex-col gap-2 p-5">
        <p>"Create New Post"</p>
        <div class="p-3 bg-white rounded-t-lg">
          // Inner border
          <p>"Title:"</p>
          <textarea
            class="p-2 w-full h-12 rounded-t-lg border border-gray-300 resize-none"
            on:input=on_input(set_post_title)
            prop:value=post_title
          ></textarea>
          <p>"Contents:"</p>
          <textarea
            class="p-2 w-full h-96 rounded-b-lg border border-gray-300 resize-none"
            on:input=on_input(set_post_contents)
            prop:value=post_contents
          ></textarea>
        </div>
        <div class="flex gap-5 justify-end">
          <label for="privateToggle" class="flex items-center cursor-pointer select-none">
            <span class="mx-2">"Private:"</span>
            <div class="relative">
              <input
                type="checkbox"
                id="privateToggle"
                class="sr-only peer"
                prop:checked=private_state
                on:change=move |_| set_private_state(!private_state())
              />
              <div class="block w-14 h-8 bg-gray-500 rounded-full"></div>
              <div class="absolute top-1 left-1 w-6 h-6 bg-white rounded-full transition peer-checked:translate-x-full peer-checked:bg-primary"></div>
            </div>
          </label>
          <label for="anonymousToggle" class="flex items-center cursor-pointer select-none">
            <span class="mx-2">"Anonymous:"</span>
            <div class="relative">
              <input
                type="checkbox"
                id="anonymousToggle"
                class="sr-only peer"
                prop:checked=anonymous_state
                on:change=move |_| set_anonymous_state(!anonymous_state())
              />
              <div class="block w-14 h-8 bg-gray-500 rounded-full"></div>
              <div class="absolute top-1 left-1 w-6 h-6 bg-white rounded-full transition peer-checked:translate-x-full peer-checked:bg-primary"></div>
            </div>
          </label>
          <button
            type="submit"
            class="p-2 text-white bg-gray-500 rounded-full hover:bg-gray-600"
            on:click=move |_| {
              edit_post_action.dispatch(());
              IS_DISPLAYED_EDIT.set(false);
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
    view! { <div class=format!("bg-[#EEEEEE] rounded-xl {}", class)>{children()}</div> }
}
