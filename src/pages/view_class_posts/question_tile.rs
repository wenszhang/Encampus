/**
 * QuestionTile component, displaying a tile for one post
 */
use crate::data::database::post_functions::Post;
use crate::resources::images::svgs::dots_icon::DotsIcon;
use crate::resources::images::svgs::lock_icon::LockIcon;
use crate::resources::images::svgs::unresolved_icon::UnresolvedIcon;

use leptos::*;
use leptos_router::A;

struct CustomTag {
    title: String,
}

enum TagPillProperties {
    Unresolved,
    Private,
    Custom(CustomTag),
}

#[component]
pub fn QuestionTile(
    post: Post,
    is_resolved: Signal<bool>,
    is_private: Signal<bool>,
) -> impl IntoView {
    view! {
        <A href=format!("{}", post.post_id)>
            <div class="relative rounded-lg shadow-lg p-6 flex flex-col \
                        items-center justify-between text-lg font-semibold h-60 \
                        w-85 transition-transform duration-300 hover:scale-105 \
                        bg-card-bg  hover:shadow-xl overflow-hidden"
            class:bg-customRed=  move || is_resolved()
            class=("hover:bg-customRed-HOVER", move || is_resolved())
            class=("hover:bg-gray-100", move || !is_resolved())
            class=("shadow-lg shadow-purple-600", is_private())>
            // class:bg-customPurple = is_private()
            // class=("border-4 border-purple-600", is_private())>

                // Card header
                <div class="text-xs absolute top-0 left-0 w-full h-12 bg-card-header rounded-t-lg shadow-md z-10 pl-6 flex gap-2 items-center">
                    //<div class="text-sm font-medium text-gray-700">Name{}</div> // Get the students name here?
                    {move || if is_resolved() {Some(view!{<TagPill props=TagPillProperties::Unresolved />})} else {None}}
                    {move || if is_private() {Some(view!{<TagPill props=TagPillProperties::Private />})} else {None}}
                    <TagPill props=TagPillProperties::Custom(CustomTag {title: "HW1".to_string()}) />

                    <button class=" flex ml-auto ">
                        <DotsIcon size="10em"/>
                    </button>

                </div>

                // Card body
                <div class="flex-grow flex items-center justify-center mt-6">
                    <p class="text-center">{post.title}</p>
                </div>
            </div>
        </A>
    }
}

#[component]
fn TagPill(props: TagPillProperties) -> impl IntoView {
    let sharedClassesAll = "px-2 py-1 rounded-full ";
    let sharedClassesWithIcon = "flex gap-2";

    match props {
        TagPillProperties::Unresolved => view! {
            <div class={[sharedClassesAll, sharedClassesWithIcon, "bg-customRed text-red-600"].join(" ")}>
                <UnresolvedIcon size="1em"/>
                Unresolved
            </div>
        },
        TagPillProperties::Private => view! {
            <div class={[sharedClassesAll, sharedClassesWithIcon, "bg-customPurple text-purple-600"].join(" ")}>
                <LockIcon size="1em"/>
                Private
            </div>
        },
        TagPillProperties::Custom(CustomTag { title }) => view! {
            <div class={[sharedClassesAll, "bg-white text-gray-600"].join(" ")}>
                {title}
            </div>
        },
    }
}
