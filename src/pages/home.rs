/**
 * Page to show the home page, first page users see when they visit the site
 */
use leptos::{component, view, IntoView};
use leptos_router::A;

/// Renders the home page
#[component]
pub fn Home() -> impl IntoView {
    view! {
      <div class="flex flex-col justify-center items-center h-screen">
        <div class="container flex flex-col items-center py-8 px-4 max-w-2xl bg-white rounded-lg shadow-md">
          <header class="flex justify-between items-center py-4">
            <h1 class="text-4xl font-bold">"Welcome to Encampus"</h1>
          </header>
          <main class="my-8 text-center">
            <p class="mb-4 text-xl">"Making Learning Accessible and Engaging for Everyone."</p>
            <A
              href="/login"
              class="py-2 px-4 font-bold text-white rounded bg-customBlue hover:bg-customBlue-HOVER"
            >
              "Login to your account"
            </A>
            <p class="mt-4 text-center">
              <a href="/about-us">"About Us"</a>
            </p>
            <p class="mt-4 text-center">
              <a href="/tutorial">"Tutorial"</a>
            </p>
          </main>
          <footer class="pt-8 mt-auto">
            <p class="text-center">"© 2024 Encampus"</p>
          </footer>
        </div>
      </div>
    }
}
