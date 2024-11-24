use leptos::{component, view, IntoView};

#[component]
pub fn AboutUs() -> impl IntoView {
    view! {
      <div class="flex flex-col justify-center items-center h-screen">
        <div class="container flex flex-col items-center px-4 mx-auto">
          <header class="flex justify-between items-center py-4">
            <h1 class="text-4xl font-bold">"About Us"</h1>
          </header>
          <main class="my-8 text-center">
            <p class="mb-4 text-xl">
              "We are a team of developers who are passionate about making learning accessible and engaging for everyone."
            </p>
            <p class="mb-4 text-xl">
              "We believe that everyone should have access to quality education, regardless of their background."
            </p>
            <p class="mb-4 text-xl">
              "Our goal is to create a platform that makes learning fun and interactive for all students."
            </p>
          </main>
          <footer class="py-8 mt-auto">
            <p class="text-center">"© 2024 Encampus"</p>
          </footer>
        </div>
      </div>
    }
}
