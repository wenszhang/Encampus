use std::time::Duration;

use chrono::NaiveDateTime;
use ev::MouseEvent;
use leptos::*;
use leptos::{create_effect, on_cleanup, set_interval};
use leptos_router::use_params;
use serde::{Deserialize, Serialize};
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

    let polls = create_resource(class_id, move |class_id| async move {
        match get_all_polls(class_id.unwrap().class_id).await {
            Ok(polls) => polls,
            Err(err) => {
                logging::error!("Failed to fetch polls: {}", err);
                vec![]
            }
        }
    });

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
                // Re-fetch the polls instead of updating manually
                polls.refetch();
            }
            set_show_modal.set(false);
        });
    };

    view! {
        <div class="flex">
            <Sidebar />
            <div class="container mx-auto my-8 p-4 flex-grow">
                <div class="flex justify-between items-center mb-6">
                    <h1 class="text-3xl font-extrabold text-gray-800">"Live Polls"</h1>
                    <Suspense fallback=move || view! { <span>"Loading..."</span> }>
                        <Show when=move || is_instructor.get().unwrap_or(false) fallback=|| ()>
                            <button
                                class="py-2 px-4 bg-gradient-to-r from-blue-500 to-indigo-600 text-white rounded hover:from-blue-600 hover:to-indigo-700"
                                on:click=move |_| set_show_modal.update(|v| *v = true)
                            >
                                "+ Create Poll"
                            </button>
                        </Show>
                    </Suspense>
                </div>
                <Suspense fallback=move || view! { <p>"Loading polls..."</p> }>
                    {move || polls.read().map(|polls| {
                        polls.iter().map(|poll| {
                            view! { <PollCard poll_data=poll.clone() /> }
                        }).collect::<Vec<_>>()
                    }).into_view()}
                </Suspense>
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

    let poll_answers = create_resource(
        move || poll_id,
        move |poll_id| async move { get_poll_answers(poll_id).await.unwrap_or_else(|_| vec![]) },
    );

    let student_answer = create_resource(
        move || (user_id, poll_id),
        move |(user_id, poll_id)| async move {
            get_student_answer(user_id, poll_id)
                .await
                .unwrap_or_else(|_| None)
        },
    );

    // Update selected answer based on student's current answer
    create_effect(move |_| {
        if let Some(answer) = student_answer.get().flatten() {
            set_selected_answer(Some(answer.clone()));
            set_has_voted(true);
        }
    });

    // Voting function
    let vote_on_answer = move |answer_text: String| {
        let old_answer = selected_answer();
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

    // Event loop to fetch poll results every 5 seconds
    create_effect(move |_| {
        if has_voted() && poll().is_active {
            let interval_callback = Closure::wrap(Box::new(move || {
                spawn_local(async move {
                    // Refetch poll data to update `poll.is_active`
                    if let Ok(updated_poll) = get_poll_by_id(poll_id).await {
                        poll.set(updated_poll);
                    }
                    poll_answers.refetch();
                });
            }) as Box<dyn Fn()>);

            let interval_id = window()
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    interval_callback.as_ref().unchecked_ref(),
                    Duration::from_secs(5).as_millis() as i32,
                )
                .unwrap();

            // Prevent the closure from being dropped
            interval_callback.forget();

            // Cleanup interval on effect teardown
            on_cleanup(move || {
                window().clear_interval_with_handle(interval_id);
            });
        }
    });

    view! {
        <div class="p-6 bg-white rounded-lg shadow-lg border border-gray-200">
            <h2 class="mb-4 text-xl font-semibold text-gray-900">{poll.get().question.clone()}</h2>
            <div class="space-y-2">
                {move || match poll_answers.get() {
                    Some(answers) => answers.iter().map(|answer| {
                        let answer_text = answer.answer.clone();
                        let vote_count = answer.voted_count;

                        // Clone `answer_text` for closures
                        let answer_text_for_class = answer_text.clone();
                        let answer_text_for_click = answer_text.clone();

                        view! {
                            <button
                                class=move || {
                                    let base_classes = "px-4 py-2 rounded-md w-full text-left";
                                    if selected_answer() == Some(answer_text_for_class.clone()) {
                                        format!("{} bg-indigo-500 text-white", base_classes)
                                    } else {
                                        format!("{} bg-gray-200 hover:bg-gray-300", base_classes)
                                    }
                                }
                                on:click=move |_| vote_on_answer(answer_text_for_click.clone())
                                disabled=move || !poll.get().is_active
                            >
                                {answer_text.clone()}
                                {move || if !poll.get().is_active {
                                    format!(" - {} votes", vote_count)
                                } else {
                                    "".to_string()
                                }}
                            </button>
                        }
                    }).collect::<Vec<_>>().into_view(),
                    None => view! { <p>"Loading options..."</p> }.into_view(),
                }}
            </div>
        </div>
    }
    .into_view()
}
#[component]
pub fn PollCreationModal(
    is_visible: Signal<bool>,
    on_close: Box<dyn Fn(MouseEvent) + 'static>,
    on_create: Box<dyn Fn(String, Vec<String>) + 'static>,
) -> impl IntoView {
    let (question, set_question) = create_signal(String::new());
    let (answers, set_answers) = create_signal(vec![String::new()]);

    // Function to add a new answer field
    let add_answer_field = move |_| {
        set_answers.update(|ans| {
            ans.push(String::new());
        });
    };

    let create_poll = move |_| {
        let question_text = question();
        let answers_list = answers()
            .iter()
            .filter(|a| !a.trim().is_empty())
            .cloned()
            .collect::<Vec<_>>();
        if !question_text.is_empty() && !answers_list.is_empty() {
            on_create(question_text, answers_list);
            set_question.set(String::new());
            set_answers.set(vec![String::new()]);
        }
    };

    view! {
        <div class=move || {
            if is_visible() { "fixed z-40 inset-0 bg-black bg-opacity-50" } else { "hidden" }
        }></div>
        <div class=move || {
            if is_visible() { "fixed z-50 inset-0 flex items-center justify-center" } else { "hidden" }
        }>
            <div class="bg-white rounded-lg shadow-xl max-w-lg w-full p-6">
                <h3 class="text-lg font-bold text-gray-900 mb-4">"Create a new poll"</h3>
                <input
                    type="text"
                    class="w-full rounded-lg border-gray-300 shadow-sm p-3 mb-4 focus:border-indigo-500 focus:ring-indigo-500"
                    placeholder="Enter your poll question"
                    prop:value=question
                    on:input=move |ev| {
                        set_question.set(event_target_value(&ev));
                    }
                />
      {move || {
        let answers_vec = answers().clone();
        answers_vec.into_iter().enumerate().map(move |(index, answer)| {
            view! {
                <input
                    type="text"
                    class="w-full rounded-lg border-gray-300 shadow-sm p-2 mb-2 focus:border-indigo-500 focus:ring-indigo-500"
                    placeholder=format!("Answer option {}", index + 1)
                    value=answer
                    on:input=move |ev| {
                        let value = event_target_value(&ev);
                        set_answers.update(move |ans| {
                            let mut new_answers = ans.clone();
                            new_answers[index] = value;
                            *ans = new_answers;
                        });
                    }
                />
            }
        }).collect::<Vec<_>>()
    }}
                <button on:click=add_answer_field class="py-2 px-4 bg-gray-300 rounded-lg hover:bg-gray-400">
                    "+ Add Answer Option"
                </button>
                <div class="flex justify-end space-x-4 mt-4">
                    <button
                        class="py-2 px-4 bg-gray-300 rounded-lg hover:bg-gray-400"
                        on:click=on_close
                    >
                        "Cancel"
                    </button>
                    <button
                        class="py-2 px-4 bg-gradient-to-r from-blue-500 to-indigo-600 text-white rounded hover:from-blue-600 hover:to-indigo-700"
                        on:click=create_poll
                        prop:disabled=move || question().trim().is_empty() || answers().iter().all(|a| a.trim().is_empty())
                    >
                        "+ Create"
                    </button>
                </div>
            </div>
        </div>
    }
}
