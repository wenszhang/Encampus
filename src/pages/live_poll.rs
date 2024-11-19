// use crate::data::database::live_poll_functions::{
//     create_poll, get_all_polls, get_poll_options, get_poll_results, update_poll,
//     vote_on_poll_option,
// };
use chrono::NaiveDateTime;
use ev::MouseEvent;
use leptos::*;

use serde::{Deserialize, Serialize};

use crate::data::database::class_functions::check_user_is_instructor;
use crate::expect_logged_in_user;
use crate::pages::view_class_posts::class::ClassId;
use leptos_router::use_params;

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Poll {
    pub id: i32,
    pub question: String,
    pub created_at: NaiveDateTime,
    pub is_active: bool,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct PollOption {
    pub id: i32,
    pub poll_id: i32,
    pub option_text: String,
    pub vote_count: i32,
}

#[component]
pub fn LivePoll() -> impl IntoView {
    let (user, _) = expect_logged_in_user!();
    let (polls, set_polls) = create_signal(Vec::<Poll>::new());
    let (show_modal, set_show_modal) = create_signal(false);
    let class_id = use_params::<ClassId>();
    // let class_id_val = class_id.get_untracked().unwrap().class_id;

    let is_instructor = create_resource(class_id, move |class_id| {
        let user_id = user().id;
        async move {
            check_user_is_instructor(user_id, class_id.unwrap().class_id)
                .await
                .unwrap_or(false)
        }
    });

    // Create a new poll and add it to the list
    let create_new_poll = move |question: String| {
        let new_poll = Poll {
            id: polls().len() as i32 + 1,
            question,
            created_at: chrono::Utc::now().naive_utc(),
            is_active: true,
        };
        set_polls.update(|p| p.push(new_poll));
        set_show_modal.update(|v| *v = false);
    };

    view! {
      <div class="container my-8 mx-auto">
        <div class="flex justify-between items-center mb-4">
          <h1 class="text-2xl font-bold">"Polls"</h1>
          <Suspense fallback=move || view! { Loading... }>
            <Show when=move || is_instructor.get().unwrap_or(false) fallback=|| ()>
              <button
                class="py-2 px-4 bg-gray-200 rounded-md hover:bg-gray-300"
                on:click=move |_| set_show_modal.update(|v| *v = true)
              >
                "Create Poll"
              </button>
            </Show>
          </Suspense>
        </div>

      </div>
      <div class="space-y-4">
        {move || {
          polls()
            .iter()
            .map(|poll| { view! { <PollCard poll=poll.clone() /> }.into_view() })
            .collect::<Vec<_>>()
        }}
        <PollCreationModal
          is_visible=show_modal.into()
          on_close=Box::new(move |_| set_show_modal.update(|v| *v = false))
          on_create=Box::new(create_new_poll)
        />
      </div>
    }
    .into_view()
}

#[component]
pub fn PollCard(poll: Poll) -> impl IntoView {
    let (poll_options, set_poll_options) = create_signal(vec![
        PollOption {
            id: 1,
            poll_id: poll.id,
            option_text: "Yes".to_string(),
            vote_count: 0,
        },
        PollOption {
            id: 2,
            poll_id: poll.id,
            option_text: "No".to_string(),
            vote_count: 0,
        },
    ]);

    let (selected_option, set_selected_option) = create_signal(None::<i32>);
    let (has_voted, set_has_voted) = create_signal(false);

    // Handle voting on an option
    let vote_on_option = move |option_id: i32| {
        // Check if the user has already voted
        if has_voted() {
            return;
        }

        set_selected_option(Some(option_id));
        set_poll_options.update(|options| {
            for option in options.iter_mut() {
                if option.id == option_id {
                    option.vote_count += 1;
                }
            }
        });
        // Set that the user has voted to prevent multiple votes
        set_has_voted(true);
    };
    let options_view = move || {
        poll_options
            .get()
            .iter()
            .map(|option| {
                let option_id = option.id;
                let option_text = option.option_text.clone();

                view! {
                  <button
                    class=move || {
                      let classes = "px-4 py-2 rounded-md w-full text-left";
                      if selected_option() == Some(option_id) {
                        format!("{} bg-indigo-500 text-white", classes)
                      } else {
                        format!("{} bg-gray-200 hover:bg-gray-300", classes)
                      }
                    }
                    on:click=move |_| vote_on_option(option_id)
                  >
                    {option_text}
                  </button>
                }
            })
            .collect::<Vec<_>>()
    };

    view! {
      <div class="p-4 bg-white rounded-lg shadow-lg">
        <h2 class="mb-2 text-lg font-bold">{poll.question}</h2>
        <div class="space-y-2">{options_view}</div>
      </div>
    }
}

#[component]
pub fn PollCreationModal(
    is_visible: Signal<bool>,
    on_close: Box<dyn Fn(MouseEvent) + 'static>,
    on_create: Box<dyn Fn(String) + 'static>,
) -> impl IntoView {
    let (question, set_question) = create_signal(String::new());

    // Handle creation of a new poll and reset form
    let create_poll = move |_| {
        if !question().is_empty() {
            on_create(question());
            set_question.set(String::new()); // Reset the form after submission
        }
    };

    view! {
      <div class=move || {
        if is_visible() { "fixed z-10 inset-0 overflow-y-auto" } else { "hidden" }
      }>
        <div class="flex justify-center items-end px-4 pt-4 pb-20 min-h-screen text-center sm:block sm:p-0">
          // Backdrop for when create poll is clicked
          <div
            class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"
            aria-hidden="true"
          />

          // Center modal contents
          <span class="hidden sm:inline-block sm:h-screen sm:align-middle" aria-hidden="true">
            &#8203;
          </span>

          // Modal panel
          <div class="inline-block overflow-hidden px-4 pt-5 pb-4 text-left align-bottom bg-white rounded-lg shadow-xl transition-all transform sm:my-8 sm:w-full sm:max-w-lg sm:align-middle">
            <div>
              <div class="mt-3 text-center sm:mt-5">
                <h3 class="text-lg font-medium leading-6 text-gray-900">"Create a new poll"</h3>
                <div class="mt-2">
                  <input
                    type="text"
                    class="w-full rounded-md border-gray-300 shadow-sm sm:text-sm focus:border-indigo-500 focus:ring-indigo-500"
                    placeholder="Enter your poll question"
                    prop:value=question
                    on:input=move |ev| {
                      set_question.set(event_target_value(&ev));
                    }
                  />
                </div>
              </div>
            </div>

            <div class="mt-5 sm:grid sm:grid-cols-2 sm:grid-flow-row-dense sm:gap-3 sm:mt-6">
              <button
                type="button"
                class="inline-flex justify-center py-2 px-4 w-full text-base font-medium text-white rounded-md border border-transparent shadow-sm sm:col-start-2 focus:ring-2 focus:ring-offset-2 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed focus:ring-custom-blue"
                // Custom blue color
                style="background-color: #1E3A8A;"
                on:click=create_poll
                prop:disabled=move || question().trim().is_empty()
              >
                "Create"
              </button>
              <button
                type="button"
                class="inline-flex justify-center py-2 px-4 mt-3 w-full text-base font-medium text-gray-700 bg-white rounded-md border border-gray-300 shadow-sm sm:col-start-1 sm:mt-0 hover:bg-gray-50 focus:ring-2 focus:ring-offset-2 focus:outline-none focus:ring-custom-blue"
                on:click=on_close
              >
                "Cancel"
              </button>
            </div>
          </div>
        </div>
      </div>
    }
}
