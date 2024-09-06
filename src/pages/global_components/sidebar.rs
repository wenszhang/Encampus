use leptos::*;
use leptos_router::A;

#[component]
pub fn Sidebar() -> impl IntoView {
    let (collapsed, set_collapsed) = create_signal(false);

    view! {
        <div class=move || if collapsed.get() { "sticky top-0 h-screen w-20 bg-gray-800 text-white" } else { "sticky top-0 h-screen w-64 bg-gray-800 text-white" }>
            <div class="flex items-center justify-center mt-10 mb-4">
                <img src="https://cdn.discordapp.com/attachments/1068270523794075678/1281628826551586947/images.jpg?ex=66dc695b&is=66db17db&hm=1a6ca3af1c813e5553660684b249b76c262d52eebac348791164a918e4da3558&" class="rounded-full w-16 h-16" alt="Instructor Image" />
            </div>
            <h1 class="text-center text-xl font-semibold" hidden=move || collapsed.get()>"Instructor"</h1>
            <div class="px-4">
                <h2 class="text-sm text-gray-400 mt-6 mb-2 uppercase tracking-widest" hidden=move || collapsed.get()>"Fall 24 Courses"</h2>
                <ul>
                    <li class="py-2">
                        <A href="/math-3210" class="block px-4 py-2 rounded-md text-white hover:bg-gray-700">"MATH 3210"</A>
                    </li>
                    <li class="py-2">
                        <A href="/cs-3220" class="block px-4 py-2 rounded-md text-white hover:bg-gray-700">"CS 3220"</A>
                    </li>
                    <li class="py-2">
                        <A href="/cs-3230" class="block px-4 py-2 rounded-md text-white hover:bg-gray-700">"CS 3230"</A>
                    </li>
                    <li class="py-2">
                        <A href="/past-courses" class="block px-4 py-2 rounded-md text-white hover:bg-gray-700">"Past Courses"</A>
                    </li>
                </ul>
                <h2 class="text-sm text-gray-400 mt-6 mb-2 uppercase tracking-widest" hidden=move || collapsed.get()>"Tools"</h2>
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
                {move || if collapsed.get() { "→" } else { "←" }}
            </button>
        </div>
    }
}
