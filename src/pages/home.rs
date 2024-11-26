/**
 * Page to show the home page, first page users see when they visit the site
 */
use leptos::{component, view, IntoView};
use leptos_router::A;

/// Renders the home page
#[component]
pub fn Home() -> impl IntoView {
    view! {
      <div class="flex flex-col justify-center items-center h-screen opacity-95">
        <div class="container flex flex-col items-center py-8 px-4 max-w-2xl bg-white rounded-lg shadow-lg">
          <header class="flex justify-between items-center py-4">
            <h1 class="text-4xl font-bold">"Welcome to Encampus"</h1>
          </header>
          <main class="my-8 text-center">
              <p class="mb-8 text-xl text-gray-700 leading-relaxed">
                "Making Learning Accessible and Engaging for Everyone."
              </p>
            <div class="flex flex-col gap-4 w-full max-w-md mx-auto">
            // Web presence
              <A
              href="/about-us"
              class="py-3 px-6 font-semibold text-gray-700 rounded-lg border-2 border-gray-300
                    hover:bg-gray-50 hover:border-gray-400 transition duration-200 
                    flex items-center justify-center gap-2"
              >
                <span>"Learn About Our Team"</span>
                <span class="text-lg"></span>
              </A>
            // login
              <A
                href="/login"
                class="py-3 px-6 font-semibold text-white rounded-lg bg-customBlue
                      hover:bg-customBlue-HOVER transition duration-200 
                      flex items-center justify-center"
              >
                "Login into your account"
              </A>
            </div>
          </main>

          <footer class="pt-8 mt-auto">
            <p class="text-center text-gray-600">"© 2024 Encampus"</p>
          </footer>
        </div>
    </div>
    }
}
