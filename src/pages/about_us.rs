use leptos::{component, view, IntoView};

#[component]
pub fn AboutUs() -> impl IntoView {
    view! {
      <div class="flex flex-col justify-center items-center h-screen">
        <div class="container flex flex-col items-center px-4 mx-auto bg-white">
          <header class="flex justify-between items-center py-4">
            <h1 class="text-4xl font-bold">"About Us"</h1>
          </header>
          <main class="my-8 text-center">
            <h1 class="mb-4 text-xl">
              "Project Overview:"
            </h1>
            <p class="mb-4 text-xl">
              "Encampus is a classroom assistance tool with the purpose of streamlining communication within classes, enabling students to ask both public and private questions outside of regular hours. Additionally, Encampus acts as a forum for students to help share insights and answer each other's questions. Another option for in-person meetings, catering to the busy schedules of diverse students on college campuses and making getting help on assignments convenient. Encampus distinguishes itself by building upon and refining the core functionalities found in similar software offering familiar tools such as Q&A forums, student participation tracking, AI responses, and live notifications. "
            </p>
            <h1 class="mb-4 text-xl">
              "Team Members:"
            </h1>
            <div class="grid grid-cols-2 gap-4">
              <img src="/images/placeholder.png" alt="Gabe" class="w-32 h-32 rounded-full" />
              <p> "Gabe is awesome" </p>
              <img src="/images/placeholder.png" alt="Jack" class="w-32 h-32 rounded-full" />
              <p> "Jack is awesome" </p>
            </div>
          </main>
          <footer class="py-8 mt-auto">
            <p class="text-center">"© 2024 Encampus"</p>
          </footer>
        </div>
      </div>
    }
}
