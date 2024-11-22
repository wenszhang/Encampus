use crate::data::database::class_functions::{get_class_name, get_users_enrolled_in_class};
use crate::data::database::post_functions::{get_resolved_questions, get_total_questions};
// use crate::data::generate_graphs::generate_answered_unanswered_histogram;
use crate::pages::view_class_posts::class::ClassId;
use leptos::*;
use leptos_router::{use_navigate, use_params};

#[component]
pub fn ClassDetails() -> impl IntoView {
    let class_id_result = use_params::<ClassId>();
    let navigate = use_navigate();

    // Fetch the class name based on class ID
    let class_name = create_local_resource(class_id_result.clone(), |class_id_result| async {
        match class_id_result {
            Ok(class_id) => get_class_name(class_id.class_id)
                .await
                .unwrap_or("Class not found".to_string()),
            Err(_) => "Invalid class ID".to_string(),
        }
    });

    // Fetch the list of users enrolled in the class with their roles
    let enrolled_users = create_local_resource(class_id_result.clone(), |class_id_result| async {
        match class_id_result {
            Ok(class_id) => get_users_enrolled_in_class(class_id.class_id)
                .await
                .unwrap_or_default(),
            Err(_) => vec![],
        }
    });

    // Canvas ID
    let canvas_id = "question-resolution-chart";

    create_effect(move |_| {
        if let Ok(class_id) = class_id_result.get() {
            spawn_local(async move {
                let total_questions =
                    get_total_questions(class_id.class_id).await.unwrap_or(0) as i32;
                let resolved = get_resolved_questions(class_id.class_id).await.unwrap_or(0) as i32;
                let unresolved = total_questions - resolved;

                // if let Err(e) =
                //     generate_answered_unanswered_histogram(canvas_id, unresolved, resolved)
                // {
                // }
            });
        }
    });

    view! {
        <div class="class-details p-8 space-y-8 bg-gray-100 min-h-screen relative">
            // Close Button
            <button
                class="absolute top-4 right-4 text-gray-600 hover:text-gray-800"
                on:click=move |_| {
                    if let Ok(class_id) = class_id_result.get() {
                        navigate(&format!("/classes/{}", class_id.class_id), Default::default());
                    }
                }>
                <span class="text-xl font-bold">"X"</span>
            </button>

            <Suspense fallback=|| view! { <p>"Loading class name..."</p> }>
                <div class="header bg-customBlue text-white text-center py-4 rounded shadow-md">
                    <h1 class="text-3xl font-bold">{class_name().unwrap_or("Class not found".to_string())}</h1>
                </div>
            </Suspense>

            <div class="content space-y-8">
                <div class="course-info bg-white p-6 rounded-lg shadow-md">
                    <h2 class="text-2xl font-semibold mb-4 text-customBlue">"Course Information"</h2>

                    <Suspense fallback=|| view! { <p>"Loading enrolled users..."</p> }>
                        <ul class="mt-4 space-y-2">
                            <For each=move || enrolled_users().unwrap_or_default() key=|user| user.user_id let:user>
                                <li class="flex justify-between items-center bg-gray-100 p-2 rounded shadow-sm">
                                    <span class="text-gray-800 font-medium">{&user.full_name}</span>
                                    <span class="text-gray-500 italic">{&user.role}</span>
                                </li>
                            </For>
                        </ul>
                    </Suspense>
                </div>

                <div class="course-stats bg-white p-6 rounded-lg shadow-md">
                    <h2 class="text-2xl font-semibold mb-4 text-customBlue">"Course Statistics"</h2>
                    // <p class="text-gray-700">"This will display various course statistics"</p>

                    // Render the chart on a canvas
                    <Suspense fallback=|| view! { <p>"Loading enrolled users..."</p> }>
                        <canvas id=canvas_id width="640" height="480"></canvas>
                    </Suspense>
                </div>
            </div>
        </div>
    }
}
