use chrono::NaiveDateTime;
use ev::MouseEvent;
use leptos::*;
use leptos_router::use_params;
use serde::{Deserialize, Serialize};

use crate::data::database::class_functions::check_user_is_instructor;
use crate::expect_logged_in_user;
use crate::pages::global_components::live_poll_sidebar::Sidebar;
use crate::pages::view_class_posts::class::ClassId;

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

    let is_instructor = create_resource(class_id, move |class_id| {
        let user_id = user().id;
        async move {
            check_user_is_instructor(user_id, class_id.unwrap().class_id)
                .await
                .unwrap_or(false)
        }
    });

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
        <div class="flex">
            // Sidebar on the left
            <Sidebar />

            // Main content area for live polls
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
                <div class="space-y-6">
                    {move || {
                        polls()
                            .iter()
                            .map(|poll| view! { <PollCard poll=poll.clone() /> })
                            .collect::<Vec<_>>()
                            .into_view()
                    }}
                    <PollCreationModal
                        is_visible=show_modal.into()
                        on_close=Box::new(move |_| set_show_modal.update(|v| *v = false))
                        on_create=Box::new(create_new_poll)
                    />
                </div>
            </div>
        </div>
    }.into_view()
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

    let vote_on_option = move |option_id: i32| {
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
        <div class="p-6 bg-white rounded-lg shadow-lg border border-gray-200">
            <h2 class="mb-4 text-xl font-semibold text-gray-900">{poll.question}</h2>
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

    let create_poll = move |_| {
        if !question().is_empty() {
            on_create(question());
            set_question.set(String::new());
        }
    };

    view! {
        // Modal Background
        <div class=move || {
            if is_visible() { "fixed z-40 inset-0 bg-black bg-opacity-50" } else { "hidden" }
        }></div>

        // Modal Content
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
                <div class="flex justify-end space-x-4">
                    <button
                        class="py-2 px-4 bg-gray-300 rounded-lg hover:bg-gray-400"
                        on:click=on_close
                    >
                        "Cancel"
                    </button>
                    <button
                    class="py-2 px-4 bg-gradient-to-r from-blue-500 to-indigo-600 text-white rounded hover:from-blue-600 hover:to-indigo-700"
                    on:click=create_poll
                        prop:disabled=move || question().trim().is_empty()
                    >
                        "+ Create"
                    </button>
                </div>
            </div>
        </div>
    }
}
