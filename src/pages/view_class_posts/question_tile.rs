/**
 * QuestionTile component, displaying a tile for one post
 */
use crate::data::database::post_functions::Post;
use crate::data::global_state::GlobalState;
use crate::resources::images::svgs::bump_icon::BumpIcon;
use crate::resources::images::svgs::dots_icon::DotsIcon;
use crate::resources::images::svgs::lock_icon::LockIcon;
use crate::resources::images::svgs::unresolved_icon::UnresolvedIcon;

use ev::MouseEvent;
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
pub fn DropDownMenu(post_author_id: i32) -> impl IntoView {
    let global_state: GlobalState = expect_context::<GlobalState>();
    // let user_role = global_state.role.get();
    // let is_authenticated = global_state.authenticated.get();
    let is_on_my_post = move || global_state.id.get() == Some(post_author_id);
    let is_professor = move || global_state.role.get() == Some("instructor".to_string());
    view! {
      <div class="pr-2 text-right">
        {move || {
          if is_professor() {
            view! {
              <button>Endorse</button>
              <button>remove</button>
              <button>pin</button>
            }
              .into_view()
          } else {
            view! {
              <div class="p-3 rounded-md w-30">
                <button class="inline-flex items-center p-1 w-full text-left text-gray-700 rounded-md hover:text-black hover:bg-gray-100">
                  <BumpIcon size="20px" />
                  <span class="ml-2">bump</span>
                </button>
              </div>
            }
              .into_view()
          }
        }}
        {move || {
          if is_on_my_post() {
            Some(
              view! {
                <button>remove</button>
                <button>pin</button>
              },
            )
          } else {
            None
          }
        }}
      </div>
    }
}

#[component]
pub fn FocusedDropdown(post_author_id: i32) -> impl IntoView {
  let global_state: GlobalState = expect_context::<GlobalState>();
  let is_on_my_post = move || global_state.id.get() == Some(post_author_id);
  let is_professor = move || global_state.role.get() == Some("instructor".to_string());

  let (menu_invisible, set_menu_invisible) = create_signal(true);


  let toggle_menu = move |e: MouseEvent| {
    e.stop_propagation();
    set_menu_invisible.update(|visible| *visible = !*visible);
  };

  view! {
    <div class="flex absolute top-0 right-2 z-20 items-center">
    <button on:click=toggle_menu class="rounded-lg bg-card-header hover:shadow-customInset">
      <DotsIcon size="36px" />
    </button>
    // Dropdown menu
    <div class=move || {
      if menu_invisible() {
        "hidden"
      } else {
        "absolute right-0 top-0 mt-7 w-30 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5"
      }}>


      <div class="pr-2 text-right">
        {move || {
          if is_professor() {
            view! {
              <button>remove</button>
              <button>pin</button>
            }
              .into_view()
          } else {
            view! {
              <div class="p-3 rounded-md w-30">
                <button class="inline-flex items-center p-1 w-full text-left text-gray-700 rounded-md hover:text-black hover:bg-gray-100"> 
                  <span class="ml-2">Resolve</span>
                </button>
                <button class="inline-flex items-center p-1 w-full text-left text-gray-700 rounded-md hover:text-black hover:bg-gray-100"> 
                  <span class="ml-2">Remove</span>
                </button>
              </div>
            }
              .into_view()
          }
        }}
        {move || {
          if is_on_my_post() {
            Some(
              view! {
                // <button>Resolve</button>
                // <button>Remove</button>
              },
            )
          } else {
            None
          }
        }}
      </div>
    </div>
    </div>
  }
}

#[component]
pub fn QuestionTile(
    post: Post,
    is_resolved: Signal<bool>,
    is_private: Signal<bool>,
) -> impl IntoView {
    let (menu_invisible, set_menu_invisible) = create_signal(true);

    let toggle_menu = move |e: MouseEvent| {
        e.stop_propagation();
        set_menu_invisible.update(|visible| *visible = !*visible);
    };

    view! {
      <div class="relative transition-transform duration-300 hover:shadow-xl hover:scale-105">
        <A href=format!("{}", post.post_id)>
          <div
            class="flex overflow-hidden flex-col justify-between items-center p-6 h-60 text-lg font-semibold rounded-lg shadow-lg w-85 bg-card-bg"
            class=("bg-customRed", move || is_resolved())
            class=("hover:bg-customRed-HOVER", move || is_resolved())
            class=("hover:bg-gray-100", move || !is_resolved())
            class:border-purple-500=is_private()
            class:border-4=is_private()
            class=("border-4 border-purple-500", is_private())
          >

            // Card header
            <div class="flex absolute top-0 left-0 z-10 gap-2 items-center pl-6 w-full h-12 text-xs rounded-t-lg shadow-md bg-card-header">
              {move || {
                if is_resolved() {
                  Some(view! { <TagPill props=TagPillProperties::Unresolved /> })
                } else {
                  None
                }
              }}
              {move || {
                if is_private() {
                  Some(view! { <TagPill props=TagPillProperties::Private /> })
                } else {
                  None
                }
              }}
              <TagPill props=TagPillProperties::Custom(CustomTag {
                title: "HW1".to_string(),
              }) />
            </div>

            // Card body
            <div class="flex flex-grow justify-center items-center mt-6">
              <p class="text-center">{post.title}</p>
            </div>
          </div>
        </A>
        <div class="flex absolute top-0 right-2 z-20 items-center">
          <button on:click=toggle_menu class="rounded-lg bg-card-header hover:shadow-customInset">
            <DotsIcon size="36px" />
          </button>
          // Dropdown menu
          <div class=move || {
            if menu_invisible() {
              "hidden"
            } else {
              "absolute right-0 top-0 mt-7 w-30 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5"
            }
          }>
            <DropDownMenu post_author_id=post.author_id />
          </div>
        </div>
      </div>
    }
}

#[component]
fn TagPill(props: TagPillProperties) -> impl IntoView {
    let sharedClassesAll = "px-2 py-1 rounded-full ";
    let sharedClassesWithIcon = "flex gap-2";

    match props {
        TagPillProperties::Unresolved => view! {
          <div class=[sharedClassesAll, sharedClassesWithIcon, "bg-customRed text-red-600"]
            .join(" ")>
            <UnresolvedIcon size="1em" />
            Unresolved
          </div>
        },
        TagPillProperties::Private => view! {
          <div class=[sharedClassesAll, sharedClassesWithIcon, "bg-customPurple text-purple-600"]
            .join(" ")>
            <LockIcon size="1em" />
            Private
          </div>
        },
        TagPillProperties::Custom(CustomTag { title }) => {
            view! { <div class=[sharedClassesAll, "bg-white text-gray-600"].join(" ")>{title}</div> }
        }
    }
}
