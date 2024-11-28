use leptos::{component, view, IntoView};

use crate::pages::about_us_pages::{
    gabe::Gabe, jack::Jack, matt::Matt, wensen::Wensen, wentao::Wentao,
};

#[component]
pub fn AboutUs() -> impl IntoView {
    view! {
      <div class="flex overflow-y-auto flex-col justify-center items-center min-h-screen bg-gray-100">
        <div class="container flex flex-col items-center py-6 px-6 mx-auto max-w-4xl bg-white rounded-lg">
          <header class="flex sticky top-0 z-10 justify-between items-center py-4 w-full bg-white">
            <h1 class="text-4xl font-bold">"About Us"</h1>
          </header>
          <main class="my-8 leading-relaxed text-left">
            <h1 class="mb-4 text-xl font-bold">"Project Overview:"</h1>
            <p class="mb-6 text-lg">
              "Encampus is a modern classroom assistance platform designed to streamline communication and foster
              collaboration within academic environments. Built with Leptos, a framework powered by Rust for exceptional 
              performance and reliability, Encampus enables students to ask public and private questions beyond regular hours, 
              bridging the gap between traditional interactions and digital convenience. It acts as a dynamic forum where 
              students can share insights, answer each other's questions, and access AI-powered responses, all while benefiting 
              from real-time notifications and participation tracking. Designed to accommodate the busy schedules of diverse college 
              students, Encampus enhances in-person and remote interactions by refining the core functionalities of similar tools, 
              providing a seamless, efficient, and innovative solution for academic success."
            </p>
            <h1 class="mb-4 text-xl font-bold">"Team Members:"</h1>
            <div class="grid grid-cols-1 gap-6 sm:grid-cols-1 md:grid-cols-2">
              {Gabe()} {Jack()} {Matt()} {Wensen()} {Wentao()}
            </div>
          </main>
          <footer class="py-8 mt-auto">
            <p class="text-center">"© 2024 Encampus"</p>
          </footer>
        </div>
      </div>
    }
}
