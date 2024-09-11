use leptos::*;
use leptos_router::A;

#[component]
pub fn Sidebar() -> impl IntoView {
    let (collapsed, set_collapsed) = create_signal(false);

    view! {
        <div class=move || if collapsed.get() { "sticky top-0 h-screen w-8 bg-gray-800 text-white flex items-center justify-center" } else { "sticky top-0 h-screen w-64 bg-gray-800 text-white" }>
            {move || {
                let courses = vec![
                    ("MATH 3210", "/math-3210"),
                    ("CS 3220", "/cs-3220"),
                    ("CS 3230", "/cs-3230"),
                    ("Past Courses", "/past-courses"),
                ]
                .into_iter()                                            // Had trouble dealing with courses expiring early, here is the solution:
                .map(|(name, url)| (name.to_string(), url.to_string())) // Convert &str to String
                .collect::<Vec<_>>();                                   // Collect into Vec<(String, String)>

                if collapsed() {
                    collapsed_view(set_collapsed).into_view()
                } else {
                    expanded_view(set_collapsed, courses).into_view() // Pass the Vec<(String, String)>
                }
            }}
        </div>
    }
}

// Collapsed view for the sidebar
fn collapsed_view(set_collapsed: WriteSignal<bool>) -> View {
    view! {
        <button class="text-white text-2xl" on:click=move |_| set_collapsed.update(|c| *c = !*c)>
            "→"
        </button>
    }
    .into_view()
}

// Expanded view for the sidebar
fn expanded_view(set_collapsed: WriteSignal<bool>, courses: Vec<(String, String)>) -> View {
    view! {
        <>
            <div class="flex items-center justify-center mt-10 mb-4">
                <img src="https://cdn.discordapp.com/attachments/1068270523794075678/1281628826551586947/images.jpg"
                    class="rounded-full w-16 h-16" alt="Instructor Image" />
            </div>
            <h1 class="text-center text-xl font-semibold">"Instructor"</h1>
            <div class="px-4">
                <h2 class="text-sm text-gray-400 mt-6 mb-2 uppercase tracking-widest">"Fall 24 Courses"</h2>
                <ul>
                    {courses.into_iter().map(|(name, url)| {
                        view! {
                            <li class="py-2">
                                <A href={url} class="block px-4 py-2 rounded-md text-white hover:bg-gray-700">{name}</A>
                            </li>
                        }
                    }).collect::<Vec<_>>()}
                </ul>
                <h2 class="text-sm text-gray-400 mt-6 mb-2 uppercase tracking-widest">"Tools"</h2>
                <ul>
                    <li class="py-2">
                        <A href="/private-messages" class="block px-4 py-2 rounded-md text-white hover:bg-gray-700">"Private Messages"</A>
                    </li>
                    <li class="py-2">
                        <A href="/course-statistics" class="block px-4 py-2 rounded-md text-white hover:bg-gray-700">"Course Statistics"</A>
                    </li>
                    <li class="py-2">
                        <A href="/pollv" class="block px-4 py-2 rounded-md text-white hover:bg-gray-700">"PollV"</A>
                    </li>
                </ul>
            </div>
            <div class="absolute bottom-4 w-full px-4">
                <A href="/account-settings" class="block w-full text-center py-2 bg-gray-700 hover:bg-gray-600 rounded-md">"Account Settings"</A>
            </div>
            <button class="absolute top-4 right-4 text-white" on:click=move |_| set_collapsed.update(|c| *c = !*c)>
                "←"
            </button>
        </>
    }
    .into_view()
}
