use leptos::*;
use leptos_router::use_params;
use crate::data::database::class_functions::get_class_name; // Function to fetch the class name
use crate::pages::view_class_posts::class::ClassId;

#[component]
pub fn ClassDetails() -> impl IntoView {
    // Retrieve the class ID from the URL parameters
    let class_id_result = use_params::<ClassId>();

    // Create a resource to fetch the class name based on class ID
    let class_name = create_local_resource(class_id_result, |class_id_result| async {
        match class_id_result {
            Ok(class_id) => get_class_name(class_id.class_id).await.unwrap_or("Class not found".to_string()),
            Err(_) => "Invalid class ID".to_string(),
        }
    });

    view! {
        <div class="class-details p-8 space-y-8 bg-gray-100 min-h-screen">
            <Suspense fallback=|| view! { <p>"Loading class name..."</p> }>
                <div class="header bg-customBlue text-white text-center py-4 rounded shadow-md">
                    <h1 class="text-3xl font-bold">{class_name().unwrap_or("Class not found".to_string())}</h1>
                </div>
            </Suspense>

            <div class="content space-y-8">
                <div class="course-info bg-white p-6 rounded-lg shadow-md">
                    <h2 class="text-2xl font-semibold mb-4 text-customBlue">"Course Information"</h2>
                    <p class="text-gray-700">"Detailed information about the course goes here. This might include the course description, instructor details, meeting times, and other relevant information."</p>
                </div>

                <div class="course-stats bg-white p-6 rounded-lg shadow-md">
                    <h2 class="text-2xl font-semibold mb-4 text-customBlue">"Course Statistics"</h2>
                    <p class="text-gray-700">"This section could display various course statistics, such as enrollment numbers, average grades, participation rates, etc."</p>
                </div>
            </div>
        </div>
    }
}
