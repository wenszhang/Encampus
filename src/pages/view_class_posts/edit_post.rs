use super::{class::ClassId, focused_post::PostDetails};
use crate::{
    data::database::post_functions::edit_post, expect_logged_in_user, on_input, pages::{global_components::rich_text_box::RichTextBox, view_class_posts::focused_post::{get_post_details, PostId}}, resources::images::svgs::{cancel_icon::CancelIcon, save_icon::SaveIcon}
};
use leptos::*;
use leptos_router::use_params;

#[component]
pub fn EditPostLoader() -> impl IntoView {
    let post_id = {
      let post_params = use_params::<PostId>();
      move || post_params().expect("Tried to render edit post without post id").post_id
    };

    let post_and_replies = create_resource(post_id, |post_id| async move {
      get_post_details(post_id).await.unwrap()
    });

    view! {
      <Suspense fallback=move || "Loading editor...">
        {move || post_and_replies().map(|(post, _)| view!{<EditPost post />})}
      </Suspense>
    }
}

#[component]
fn EditPost(post: PostDetails) -> impl IntoView {
    let (user, _) = expect_logged_in_user!();
    let class_id = {
      let class_params = use_params::<ClassId>();
      move || class_params().expect("Tried to render edit post without class id").class_id
    };

    let (post_title, set_post_title) = create_signal(
        post.title,
    );
    let (post_contents, set_post_contents) = create_signal(
        post.contents
    );
    let (private_state, set_private_state) = create_signal(post.private);
    let (anonymous_state, set_anonymous_state) = create_signal(post.anonymous);

    let edit_post_action = create_action(move |(user_id, class_id, post_id, post_title, post_contents, private, anonymous): &(i32, i32, i32, String, String, bool, bool)| {
        let user_id = *user_id;
        let class_id = *class_id;
        let post_id = *post_id;
        let post_title = post_title.clone();
        let post_contents = post_contents.clone();
        let private = *private;
        let anonymous = *anonymous;
        async move {
            match edit_post(
                post_id,
                post_title,
                post_contents,
                user_id,
                private,
                anonymous,
            )
            .await
            {
                Ok(_) => {
                    let navigate = leptos_router::use_navigate();
                    navigate(
                        format!("/classes/{}/{}", class_id, post_id)
                            .as_str(),
                        Default::default(),
                    );
                }
                Err(e) => {
                    logging::error!("Error editing post: {:?}", e);
                }
            }
        }
    });

    view! {
      <DarkenedCard class="flex flex-col gap-2 p-5">
        <p>"Edit Post"</p>
        <div class="p-3 bg-white rounded-t-lg">
          // Inner border
          <p>"Title:"</p>
          <textarea
            class="p-2 w-full h-12 rounded-t-lg border border-gray-300 resize-none"
            on:input=on_input!(set_post_title)
            prop:value=post_title
          ></textarea>
          <p>"Contents:"</p>
          <RichTextBox
            id="edit_post_rich_text_box".to_string()
            set_value=set_post_contents
            value=post_contents
          />
        </div>

          <div class="flex gap-5 justify-end">
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
      <div class="flex gap-5 justify-end">
      <button
        class="ml-4 py-2 px-4 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:outline-none bg-red-500 hover:bg-red-600 focus:ring-offset-red-500 flex items-center gap-2"
        type="button"
        on:click=move |_| {
          let navigate = leptos_router::use_navigate();
          navigate(
            format!("/classes/{}", class_id()).as_str(),
            Default::default(),
          );
        }
      >
      <CancelIcon size="1em"/>
        "Cancel"
      </button>
    </div>
      <button
      type="submit"
        class="py-3 px-4 text-white rounded-full focus:ring-2 focus:ring-offset-2 focus:ring-offset-coolBlue bg-coolBlue hover:bg-coolBlue-HOVER focus:outline-none inline-flex items-center gap-2"
        on:click=move |_| {
          edit_post_action.dispatch((user().id, class_id(), post.post_id, post_title(), post_contents(), private_state(), anonymous_state()));
        }
      >
        Save Changes
        <SaveIcon size="1.5em"/>
      </button>
          </div>


      </DarkenedCard>
    }
}

#[component]
fn DarkenedCard(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    view! { <div class=format!("bg-[#EEEEEE] rounded-xl {}", class)>{children()}</div> }
}
