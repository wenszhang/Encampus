use leptos::{component, view, IntoView};

#[component]
pub fn AboutUs() -> impl IntoView {
    view! {
      <div class="flex flex-col justify-center items-center h-screen">
        <div class="container flex flex-col items-center px-4 mx-auto bg-white rounded-lg">
          <header class="flex justify-between items-center py-4">
            <h1 class="text-4xl font-bold">"About Us"</h1>
          </header>
          <main class="my-8 text-left">
            <h1 class="mb-4 text-xl">"Project Overview:"</h1>
            <p class="mb-4 text-xl">
              "Encampus is a modern classroom assistance platform designed to streamline communication and foster
              collaboration within academic environments. Built with Leptos, a framework powered by Rust for exceptional 
              performance and reliability, Encampus enables students to ask public and private questions beyond regular hours, 
              bridging the gap between traditional interactions and digital convenience. It acts as a dynamic forum where 
              students can share insights, answer each other's questions, and access AI-powered responses, all while benefiting 
              from real-time notifications and participation tracking. Designed to accommodate the busy schedules of diverse college 
              students, Encampus enhances in-person and remote interactions by refining the core functionalities of similar tools, 
              providing a seamless, efficient, and innovative solution for academic success."
            </p>
            <h1 class="mb-4 text-xl">"Team Members:"</h1>
            <div class="grid grid-cols-2 gap-4">
              <img src="/images/placeholder.png" alt="Gabe" class="w-32 h-32 rounded-full" />
              <div>
                <div class="border-b border-b-black">
                  <p class="text-xl">"Gabriel Famodu"</p>
                  <p>"Contact Info"</p>
                </div>
                <p>"Bio Here"</p>
              </div>
              <div>
                <div class="border-b border-b-black">
                  <p class="text-xl">"Jack Shunn"</p>
                  <p>"Contact Info"</p>
                </div>
                <p>"Bio Here"</p>
              </div>
              <img src="/images/placeholder.png" alt="Jack" class="w-32 h-32 rounded-full" />
            </div>
          </main>
          <footer class="py-8 mt-auto">
            <p class="text-center">"© 2024 Encampus"</p>
          </footer>
        </div>
      </div>
    }
}
