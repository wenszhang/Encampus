use ev::MouseEvent;
use gloo_timers::callback::Interval;
use leptos::*;
use leptos::{create_effect, on_cleanup};
use leptos_router::use_params;
use std::time::Duration;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

use crate::data::database::class_functions::check_user_is_instructor;
use crate::data::database::live_poll_functions::*;
use crate::expect_logged_in_user;
use crate::pages::global_components::live_poll_sidebar::Sidebar;
use crate::pages::view_class_posts::class::ClassId;

#[component]
pub fn LivePoll() -> impl IntoView {
    let (user, _) = expect_logged_in_user!();
    let class_id = use_params::<ClassId>();

    // Modified polls resource to filter for students
    let polls_resource = create_resource(class_id, move |class_id| async move {
        let course_id = class_id.unwrap().class_id;
        // First check if user is instructor
        let is_instructor = check_user_is_instructor(user().id, course_id)
            .await
            .unwrap_or(false);

        // Then get and filter polls accordingly
        match get_all_polls(course_id, user().id).await {
            Ok(all_polls) => {
                if is_instructor {
                    all_polls
                } else {
                    // Only show active polls to students
                    all_polls
                        .into_iter()
                        .filter(|poll| poll.poll.is_active)
                        .collect()
                }
            }
            Err(err) => {
                logging::error!("Failed to fetch polls: {}", err);
                vec![]
            }
        }
    });

    // Interval callback to fetch poll results every 3 seconds
    create_effect(move |_| {
        Interval::new(3000, move || {
            polls_resource.refetch();
        })
    });

    let polls = create_memo(move |_| polls_resource.get());

    // Rest of your component remains exactly the same
    let (show_modal, set_show_modal) = create_signal(false);

    let is_instructor = create_resource(class_id, move |class_id| {
        let user_id = user().id;
        async move {
            check_user_is_instructor(user_id, class_id.unwrap().class_id)
                .await
                .unwrap_or(false)
        }
    });

    let create_new_poll = move |question: String, answers: Vec<String>| {
        spawn_local(async move {
            let course_id = class_id().unwrap().class_id;
            if let Ok(_) = create_poll(question, course_id, answers).await {
                polls_resource.refetch();
            }
            set_show_modal.set(false);
        });
    };

    // Your existing view code remains exactly the same
    view! {
      <div class="flex">
        <Sidebar />
        <div class="container flex-grow p-4 my-8 mx-auto">
          <div class="flex justify-between items-center mb-6">
            <h1 class="text-3xl font-extrabold text-gray-800">"Live Polls"</h1>
            <Suspense fallback=move || view! { <span>"Loading..."</span> }.into_view()>
              <Show
                when=move || is_instructor.get().unwrap_or(false)
                fallback=|| view! { <div></div> }.into_view()
              >
                <button
                  class="py-2 px-4 text-white bg-gradient-to-r from-blue-500 to-indigo-600 rounded hover:from-blue-600 hover:to-indigo-700"
                  on:click=move |_| set_show_modal.update(|v| *v = true)
                >
                  "+ Create Poll"
                </button>
              </Show>
            </Suspense>
          </div>
          <Transition fallback=move || {
            view! { <p>"Loading polls..."</p> }.into_view()
          }>
            {move || {
              polls
                .get()
                .map(|polls| {
                  polls
                    .iter()
                    .map(|poll| view! { <PollCard poll_data=poll.poll.clone() /> }.into_view())
                    .collect::<Vec<_>>()
                })
                .into_view()
            }}
          </Transition>
          <PollCreationModal
            is_visible=show_modal.into()
            on_close=Box::new(move |_| set_show_modal.update(|v| *v = false))
            on_create=Box::new(create_new_poll)
          />
        </div>
      </div>
    }.into_view()
}

#[component]
pub fn PollCard(poll_data: Poll) -> impl IntoView {
    let poll = create_rw_signal(poll_data);
    let poll_id = poll().id;
    let (user, _) = expect_logged_in_user!();
    let user_id = user().id;

    let (selected_answer, set_selected_answer) = create_signal(None::<String>);
    let (has_voted, set_has_voted) = create_signal(false);
    let (is_deleted, set_is_deleted) = create_signal(false);

    let is_instructor = create_resource(
        move || user_id,
        move |user_id| {
            let class_id = poll().course_id;
            async move {
                check_user_is_instructor(user_id, class_id)
                    .await
                    .unwrap_or(false)
            }
        },
    );

    let poll_answers = create_resource(
        move || poll_id,
        move |poll_id| async move { get_poll_answers(poll_id).await.unwrap_or_else(|_| vec![]) },
    );

    let student_answer = create_resource(
        move || (user_id, poll_id),
        move |(user_id, poll_id)| async move {
            get_student_answer(user_id, poll_id).await.unwrap_or(None)
        },
    );

    // Update selected answer based on student's current answer
    create_effect(move |_| {
        if let Some(answer) = student_answer.get().flatten() {
            set_selected_answer(Some(answer.clone()));
            set_has_voted(true);
        }
    });

    // Delete poll action
    let delete_poll = move |_: MouseEvent| {
        let poll_id = poll_id;
        spawn_local(async move {
            match delete_poll(poll_id).await {
                Ok(_) => {
                    set_is_deleted(true);
                }
                Err(e) => {
                    logging::error!("Failed to delete poll: {}", e);
                }
            }
        });
    };

    // Toggle poll status action
    let toggle_poll_status = move |_: MouseEvent| {
        let poll_id = poll_id;
        let new_status = !poll().is_active;

        spawn_local(async move {
            match set_poll_active_status(poll_id, new_status).await {
                Ok(updated_poll) => {
                    poll.set(updated_poll);
                }
                Err(e) => {
                    logging::error!("Failed to update poll status: {}", e);
                }
            }
        });
    };

    // Updated voting function
    let vote_on_answer = move |answer_text: String| {
        // Check if poll is inactive or user is instructor - prevent voting in either case
        if !poll().is_active || is_instructor.get().unwrap_or(false) {
            return;
        }

        let old_answer = selected_answer();
        // Don't do anything if clicking the same answer
        if old_answer.as_ref() == Some(&answer_text) {
            return;
        }
        let new_answer = answer_text.clone();
        let user_id = user().id;
        let poll_id = poll_id;
        spawn_local(async move {
            if let Ok(_) =
                vote_on_poll_answer(user_id, poll_id, new_answer.clone(), old_answer).await
            {
                set_selected_answer(Some(new_answer));
                set_has_voted(true);
                poll_answers.refetch();
            }
        });
    };

    // Interval callback to fetch poll results every 2 seconds
    create_effect(move |_| {
        if has_voted() && poll().is_active {
            // creates a new interval, following
            Some(Interval::new(2000, move || {
                spawn_local(async move {
                    if let Ok(updated_poll) = get_poll_by_id(poll_id).await {
                        poll.set(updated_poll);
                    }
                    poll_answers.refetch();
                });
            }))
        } else {
            None
        }
    });

    view! {
      <Show when=move || !is_deleted() fallback=|| view! { <div></div> }>
        <div class="relative p-6 bg-white rounded-lg border border-gray-200 shadow-lg group">
          <Show when=move || is_instructor.get().unwrap_or(false) fallback=|| ()>
            <div class="flex absolute top-2 right-2 items-center space-x-2">
              <span class=move || {
                if poll().is_active {
                  "text-green-500 font-medium mr-2"
                } else {
                  "text-red-500 font-medium mr-2"
                }
              }>{move || if poll().is_active { "Active" } else { "Inactive" }}</span>

              <button
                class="p-2 text-gray-500 rounded-full opacity-0 transition-opacity group-hover:opacity-100 hover:text-blue-500 hover:bg-gray-100"
                on:click=toggle_poll_status
                title=move || if poll().is_active { "End Poll" } else { "Reactivate Poll" }
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  class="w-5 h-5"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  {move || {
                    if poll().is_active {
                      view! {
                        <path
                          fill-rule="evenodd"
                          d="M10 18a8 8 0 100-16 8 8 0 000 16zM8 7a1 1 0 00-1 1v4a1 1 0 002 0V8a1 1 0 00-1-1zm4 0a1 1 0 00-1 1v4a1 1 0 002 0V8a1 1 0 00-1-1z"
                          clip-rule="evenodd"
                        />
                      }
                    } else {
                      view! {
                        <path
                          fill-rule="evenodd"
                          d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z"
                          clip-rule="evenodd"
                        />
                      }
                    }
                  }}
                </svg>
              </button>

              <button
                class="p-2 text-gray-500 rounded-full opacity-0 transition-opacity group-hover:opacity-100 hover:text-red-500 hover:bg-gray-100"
                on:click=delete_poll
                title="Delete poll"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  class="w-5 h-5"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path
                    fill-rule="evenodd"
                    d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                    clip-rule="evenodd"
                  />
                </svg>
              </button>
            </div>
          </Show>

          <h2 class="mb-4 text-xl font-semibold text-gray-900">{poll.get().question.clone()}</h2>

          // Updated vote message section with conditional text
          <Show when=move || !is_instructor.get().unwrap_or(false) && has_voted() fallback=|| ()>
            <p class="mb-4 font-medium">
              {move || {
                if poll().is_active {
                  view! { <span class="text-green-600">"You have voted on this poll"</span> }
                } else {
                  view! { <span class="text-blue-600">"Voting has ended"</span> }
                }
              }}
            </p>
          </Show>

          <div class="space-y-2">
            {move || match poll_answers.get() {
              Some(answers) => {
                answers
                  .iter()
                  .map(|answer| {
                    let answer_text = answer.answer.clone();
                    let vote_count = answer.voted_count;
                    let answer_text_for_class = answer_text.clone();
                    let answer_text_for_click = answer_text.clone();

                    view! {
                      <button
                        class=move || {
                          let base_classes = "px-4 py-2 rounded-md w-full text-left";
                          if selected_answer() == Some(answer_text_for_class.clone()) {
                            format!("{} bg-customBlue text-white", base_classes)
                          } else {
                            format!("{} bg-gray-200 hover:bg-gray-300", base_classes)
                          }
                        }
                        on:click=move |_| vote_on_answer(answer_text_for_click.clone())
                        disabled=move || {
                          !poll.get().is_active || is_instructor.get().unwrap_or(false)
                        }
                      >
                        <div class="flex justify-between items-center">
                          <span>{answer_text.clone()}</span>
                          <Show when=move || {
                            !poll.get().is_active || is_instructor.get().unwrap_or(false)
                          }>
                            <span class="ml-2 text-sm">{format!("{} votes", vote_count)}</span>
                          </Show>
                        </div>
                      </button>
                    }
                  })
                  .collect::<Vec<_>>()
                  .into_view()
              }
              None => view! { <p>"Loading options..."</p> }.into_view(),
            }}
          </div>
        </div>
      </Show>
    }
}
#[derive(Clone)]
struct AnswerField {
    content: RwSignal<String>,
    key: i32,
}

#[component]
pub fn PollCreationModal(
    is_visible: Signal<bool>,
    on_close: Box<dyn Fn(MouseEvent) + 'static>,
    on_create: Box<dyn Fn(String, Vec<String>) + 'static>,
) -> impl IntoView {
    let (question, set_question) = create_signal(String::new());
    let (counter, set_counter) = create_signal(0);
    let (answers, set_answers) = create_signal(vec![AnswerField {
        content: RwSignal::new(String::new()),
        key: {
            let key = counter.get_untracked();
            set_counter.update(|i| *i += 1);
            key
        },
    }]);

    // Function to add a new answer field
    let add_answer_field = move |_| {
        set_answers.update(|ans| {
            ans.push(AnswerField {
                content: RwSignal::new(String::new()),
                key: {
                    let key = counter.get_untracked();
                    set_counter.update(|i| *i += 1);
                    key
                },
            });
        });
    };

    let create_poll = move |_| {
        let question_text = question();
        let answers_list = answers()
            .iter()
            .map(|answer_signal| answer_signal.content.get_untracked())
            .filter(|a| !a.trim().is_empty())
            .collect::<Vec<_>>();
        if !question_text.is_empty() && !answers_list.is_empty() {
            on_create(question_text, answers_list);
            set_question.set(String::new());
            set_answers.set(vec![AnswerField {
                content: RwSignal::new(String::new()),
                key: {
                    let key = counter.get_untracked();
                    set_counter.update(|i| *i += 1);
                    key
                },
            }]);
        }
    };

    view! {
      <div class=move || {
        if is_visible() { "fixed z-40 inset-0 bg-black bg-opacity-50" } else { "hidden" }
      }></div>
      <div class=move || {
        if is_visible() { "fixed z-50 inset-0 flex items-center justify-center" } else { "hidden" }
      }>
        <div class="p-6 w-full max-w-lg bg-white rounded-lg shadow-xl">
          <h3 class="mb-4 text-lg font-bold text-gray-900">"Create a new poll"</h3>
          <input
            type="text"
            class="p-3 mb-4 w-full rounded-lg shadow-sm border-customBlue focus:border-customBlue-HOVER focus:ring-customBlue-HOVER"
            placeholder="Enter your poll question"
            prop:value=question
            on:input=move |ev| {
              set_question.set(event_target_value(&ev));
            }
          />

          <For
            each=answers
            key=|answer| answer.key
            children=|answer| {
              view! {
                <input
                  type="text"
                  class="p-2 mb-2 w-full rounded-lg border-gray-300 shadow-sm border-customBlue focus:border-customBlue-HOVER focus:ring-customBlue-HOVER"
                  placeholder="Answer option"
                  value=answer.content
                  on:input=move |ev| {
                    let value = event_target_value(&ev);
                    answer.content.set(value);
                  }
                />
              }
            }
          />
          <button
            on:click=add_answer_field
            class="py-2 px-4 bg-gray-300 rounded-lg hover:bg-gray-400"
          >
            "+ Add Answer Option"
          </button>
          <div class="flex justify-end mt-4 space-x-4">
            <button class="py-2 px-4 bg-gray-300 rounded-lg hover:bg-gray-400" on:click=on_close>
              "Cancel"
            </button>
            <button
              class="py-2 px-4 text-white bg-gradient-to-r from-blue-500 to-indigo-600 rounded hover:from-blue-600 hover:to-indigo-700"
              on:click=create_poll
              prop:disabled=move || {
                question().trim().is_empty()
                  || answers().iter().any(|a| a.content.get().trim().is_empty())
              }
            >
              "+ Create"
            </button>
          </div>
        </div>
      </div>
    }
}
