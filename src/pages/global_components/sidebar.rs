use crate::data::database::class_functions::get_class_list;
use crate::data::database::class_functions::ClassInfo;
use crate::data::global_state::GlobalState;
use leptos::*;
use leptos_router::A;

#[component]
pub fn Sidebar() -> impl IntoView {
    let (collapsed, set_collapsed) = create_signal(false);

    let courses = create_resource(
        || {},
        |_| async { get_class_list().await.unwrap_or_default() },
    );

    view! {
        <div class=move || if collapsed.get() { "sticky top-0 h-screen w-8 bg-gray-800 text-white flex items-center justify-center" } else { "sticky top-0 h-screen w-64 bg-gray-800 text-white" }>
            {move || {
                if collapsed() {
                    collapsed_view(set_collapsed).into_view()
                } else {
                    expanded_view(set_collapsed, courses).into_view()
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
fn expanded_view(set_collapsed: WriteSignal<bool>, courses: Resource<(), Vec<ClassInfo>>) -> View {
    let global_state = expect_context::<GlobalState>(); // Access global state

    view! {
        <div class="flex flex-col h-full">
            // Profile Image and User Info
            <div class="flex items-center justify-center mt-10 mb-4">
                <img src="https://static.vecteezy.com/system/resources/thumbnails/009/292/244/small/default-avatar-icon-of-social-media-user-vector.jpg"
                    class="rounded-full w-16 h-16" alt="Profile Image" />
            </div>

            // Reactive Name and Role
            <h1 class="text-center text-2xl font-bold">
                {move || {
                    let first_name = global_state.first_name.get();
                    let last_name = global_state.last_name.get();
                    format!("{} {}", 
                        first_name.unwrap_or_else(|| "".to_string()), 
                        last_name.unwrap_or_else(|| "".to_string())
                    )
                }}
            </h1>
            <h2 class="text-center text-lg font-semibold text-gray-500">
                {move || global_state.role.get().unwrap_or_else(|| "".to_string())}
            </h2>

            <div class="flex-grow overflow-y-auto px-4 mt-6 custom-scrollbar">
                <h2 class="text-sm text-gray-400 mb-2 uppercase tracking-widest">"Fall 24 Courses"</h2>
                <ul>
                    <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                        <For each=move || courses().unwrap_or_default() key=|class| class.id let:class>
                            <li class="py-2">
                                <A href={format!("/classes/{}", class.id)} target="_self" class="block px-4 py-2 rounded-md text-white hover:bg-gray-700">{class.name}</A>
                            </li>
                        </For>
                    </Suspense>
                </ul>

                <h2 class="text-sm text-gray-400 mt-6 mb-2 uppercase tracking-widest">"Tools"</h2>
                <ul>
                    <li class="py-2">
                        <A href="/classes" class="block px-4 py-2 rounded-md text-white hover:bg-gray-700">"Private Messages"</A>
                    </li>
                    <li class="py-2">
                        <A href="/classes" class="block px-4 py-2 rounded-md text-white hover:bg-gray-700">"Course Statistics"</A>
                    </li>
                    <li class="py-2">
                        <A href="/classes" class="block px-4 py-2 rounded-md text-white hover:bg-gray-700">"PollV"</A>
                    </li>
                </ul>
            </div>

            // Account Settings Button
            <div class="w-full px-2 py-2 bg-gray-700 hover:bg-gray-600 rounded-md">
                <A href="/classes" class="block w-full text-center py-1 text-sm text-white">"Account Settings"</A>
            </div>

            // Collapse Button
            <button class="absolute top-4 right-4 text-white" on:click=move |_| set_collapsed.update(|c| *c = !*c)>
                "←"
            </button>
        </div>
    }
    .into_view()
}
