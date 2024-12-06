use crate::data::database::class_functions::{get_class_name, get_users_enrolled_in_class};
use crate::data::database::post_functions::{get_resolved_questions, get_total_questions};
use crate::data::generate_graphs::generate_answered_unanswered_histogram;
use crate::pages::view_class_posts::class::ClassId;
use leptos::*;
use leptos_router::{use_navigate, use_params};

#[component]
pub fn ClassDetails() -> impl IntoView {
    let class_id = {
        let class_params = use_params::<ClassId>();
        move || class_params().expect("Tried to render class details without class id").class_id
    };
    let navigate = use_navigate();

    // Fetch the class name based on class ID
    let class_name = create_local_resource(class_id, |class_id| async move {
        get_class_name(class_id)
            .await
            .unwrap_or("Class not found".to_string())
    });

    // Fetch the list of users enrolled in the class with their roles
    let enrolled_users = create_local_resource(class_id, |class_id| async move {
        get_users_enrolled_in_class(class_id)
            .await
            .unwrap_or_default()
    });

    // Canvas ID
    let canvas_id = "question-resolution-chart";

    create_effect(move |_| {
        let class_id = class_id(); 
        spawn_local(async move {
            let total_questions =
                get_total_questions(class_id).await.unwrap_or(0) as i32;
            let resolved = get_resolved_questions(class_id).await.unwrap_or(0) as i32;
            let unresolved = total_questions - resolved;

            let _ = generate_answered_unanswered_histogram(canvas_id, unresolved, resolved);
        });
    });

    view! {
        <div class="fixed inset-0 bg-black bg-opacity-50 z-40 flex items-center justify-center p-4">
          // fix view for cleaner pop up
            <div class="relative bg-white rounded-xl w-11/12 max-w-4xl max-h-[90vh] overflow-hidden shadow-2xl">
                // Close button
                <button
                class="absolute top-4 right-4 w-8 h-8 bg-red-500 text-white rounded-full flex items-center justify-center hover:bg-red-600 transition-colors duration-200 shadow-lg z-50"
                on:click=move |_| navigate(format!("/classes/{}", class_id()).as_str(), Default::default())
                >
                    <span class="text-xl font-bold leading-none">"×"</span>
                </button>

                // Header
                <Suspense fallback=|| view! { <div class="h-16 bg-customBlue animate-pulse"></div> }>
                    <div class="w-full py-6 px-8 bg-customBlue text-center">
                        <h1 class="text-2xl font-bold text-white">
                            {class_name().unwrap_or("Class not found".to_string())}
                        </h1>
                    </div>
                </Suspense>

                // Scrollable content
                <div class="overflow-y-auto p-8 space-y-6 max-h-[calc(90vh-4rem)]">
                    // Course Information Section
                    <div class="bg-gray-50 rounded-lg shadow-sm border border-gray-100">
                        <div class="p-6">
                            <h2 class="text-xl font-semibold text-customBlue mb-6">"Course Information"</h2>

                            <Suspense fallback=|| view! { <div class="animate-pulse h-40 bg-gray-200 rounded"></div> }>
                                <ul class="space-y-3">
                                    <For
                                        each=move || enrolled_users().unwrap_or_default()
                                        key=|user| user.user_id
                                        let:user
                                    >
                                        <li class="flex justify-between items-center p-3 bg-white rounded border border-gray-100 hover:border-gray-200 transition-colors duration-200">
                                            <span class="font-medium text-gray-800">{&user.full_name}</span>
                                            <span class="text-sm px-3 py-1 bg-gray-100 rounded-full text-gray-600">{&user.role}</span>
                                        </li>
                                    </For>
                                </ul>
                            </Suspense>
                        </div>
                    </div>

                    // Course Statistics Section
                    <div class="bg-gray-50 rounded-lg shadow-sm border border-gray-100">
                        <div class="p-6">
                            <h2 class="text-xl font-semibold text-customBlue mb-6">"Course Statistics"</h2>
                            <Suspense fallback=|| view! { <div class="animate-pulse h-[480px] bg-gray-200 rounded"></div> }>
                                <div class="bg-white p-4 rounded border border-gray-100">
                                    <canvas id=canvas_id width="640" height="480"></canvas>
                                </div>
                            </Suspense>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
