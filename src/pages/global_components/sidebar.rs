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
                                                        // Clone global_state so it can be used in multiple closures
    let global_state_clone_for_first_name = global_state.clone();
    let global_state_clone_for_last_name = global_state.clone();
    let global_state_clone_for_role = global_state.clone();
    let first_name = global_state_clone_for_first_name.first_name.get();
    let last_name = global_state_clone_for_last_name.last_name.get();
    let role = global_state_clone_for_role.role.get();

    view! {
        <>
            <div class="flex items-center justify-center mt-10 mb-4">
                <img src="https://static.vecteezy.com/system/resources/thumbnails/009/292/244/small/default-avatar-icon-of-social-media-user-vector.jpg"
                    class="rounded-full w-16 h-16" alt="Profile Image" />
            </div>

            // Name and role
            <h1 class="text-center text-2xl font-bold">{first_name} " " {last_name}</h1> 
            <h2 class="text-center text-lg font-semibold text-gray-500">{role}</h2>

            <div class="px-4">
                <h2 class="text-sm text-gray-400 mt-6 mb-2 uppercase tracking-widest">"Fall 24 Courses"</h2>
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
