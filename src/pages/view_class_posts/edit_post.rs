
#[component]
pub fn EditPost(class_id: i32,
    posts: Resource<PostFetcher, Vec<Post>>,
    post: PostDetails
) -> impl IntoView {
    let global_state: GlobalState = expect_context::<GlobalState>();
    let (post_title, set_post_title) = create_signal(post.title);
    let (post_contents, set_post_contents) = create_signal(post.contents);
    let (private_state, set_private_state) = create_signal(post.private);
    let (anonymous_state, set_anonymous_state) = create_signal(post.anonymous);
    
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
                if post_title().is_empty() || post_contents().is_empty() {
                  return;
                }
                // edit post action call
              }
            >
              "Post"
            </button>
          </div>
        </DarkenedCard>
      }    
}
