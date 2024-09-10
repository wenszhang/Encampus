use super::question_tile::QuestionTile;
use crate::data::database::announcement_functions::get_announcement_list;
use crate::data::database::class_functions::get_class_name;
use crate::data::database::post_functions::get_posts;
use crate::data::database::post_functions::PostFetcher;
use crate::data::global_state::GlobalState;
use crate::pages::global_components::announcements::Announcements;
use crate::pages::global_components::header::Header;
use crate::pages::global_components::sidebar::Sidebar;
use crate::pages::view_class_posts::create_post::CreatePost;
use crate::resources::images::svgs::filter_icon::FilterIcon;
use crate::resources::images::svgs::information_icon::InformationIcon;
use crate::resources::images::svgs::magnifying_glass::MagnifyingGlass;

use leptos::*;
use leptos_router::{use_params, Outlet, Params, A};

#[derive(Params, PartialEq, Clone)]
pub struct ClassId {
    pub class_id: i32,
}

/**
 * Page getting and displaying all posts in a class
 */
#[component]
pub fn ClassPage() -> impl IntoView {
    let global_state = expect_context::<GlobalState>();
    // Fetch class id from route in the format of "class/:class_id"
    let class_id = use_params::<ClassId>();

    let post_data = PostFetcher {
        class_id: class_id.get().unwrap().class_id,
        user_id: global_state.id.get_untracked().unwrap_or_default(),
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
    });

    let (is_visible, set_is_visible) = create_signal(false);

    view! {
        <div class="flex">
            <Sidebar/>
            <div class="flex-1">
                <Suspense fallback=move || view! { } >
                    <Header text={class_name().unwrap_or_default()} logo={None} class_id={Signal::derive(move || class_id().ok().map(|id| id.class_id))}/>
                </Suspense>
                <span class="inline-flex items-baseline ml-5">
                    <button class="pr-1 pt-7">
                        <InformationIcon size="20px"/>
                    </button>
                    <h3 class="text-s pb-1"> "Help"</h3>
                </span>
                <div class="flex justify-center pt-8 mx-20">
                    <div class="flex items-center justify-center">
                        <div class="relative p-2 rounded-full border border-gray-300 shadow-lg focus-within:border-blue-500 w-[35rem] flex items-center bg-white">
                            <input type="text" placeholder="Search posts by keywords..." class="pl-5 pr-24 w-full border-none focus:outline-none bg-white"/>
                            <button class="absolute flex items-center justify-between inset-y-0 right-12 top-1 bg-gray-300 text-white  rounded-full py-1 px-10  hover:bg-gray-400" style="height: 30px;">
                                <p class="text-xs pr-2"> "Filter Posts" </p>
                                <FilterIcon size="20px"/>
                            </button>
                            <button class="absolute inset-y-0 right-0 pr-4 flex items-center">
                                <MagnifyingGlass size="21px"/>
                            </button>
                        </div>
                    </div>
                    <button class="bg-customBlue hover:bg-customBlue-HOVER text-white py-2 px-4 rounded-full focus:outline-none focus:ring-2 focus:ring-offset-customBlue focus:ring-offset-2"
                        on:click=move |_| set_is_visible(!is_visible())>
                        { move || if is_visible() { "Cancel" } else { "Post +" } }
                    </button>
                </div>
                <div class="flex align mx-20 my-10 flex-col gap-4">
                    <Show when=move || is_visible() fallback=|| ()>
                        <CreatePost/>
                    </Show>
                    <Outlet/> // Gets replaced with the focused post if there's one in the route. See router
                    // announcements section
                    <Suspense fallback=move || view! { <p>"Loading announcements..."</p> }>
                        { move || {
                            let ann_list = announcements().unwrap_or_default();
                            view! { <Announcements announcements={ann_list} /> }
                        }}
                    </Suspense>
                    <div class="grid grid-cols-3 gap-4">
                        <Suspense fallback=move || view! { <p>"Loading..."</p> } >
                            <For each=move || posts().unwrap_or_default() key=|post| post.post_id let:post>
                                {let private = post.private;
                                post.resolved.then(|| view! { <QuestionTile post={post.clone()} is_resolved=(|| false).into_signal() is_private=(move || private).into_signal()  />}).unwrap_or_else(|| view! { <QuestionTile post={post.clone()} is_resolved=(|| true).into_signal()  is_private=(move || private).into_signal()  />})}
                            </For>
                        </Suspense>
                    </div>
                </div>
            </div>
        </div>
    }
}
