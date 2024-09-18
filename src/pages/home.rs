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
        <div class="container flex flex-col items-center px-4 mx-auto">
          <header class="flex justify-between items-center py-4">
            <h1 class="text-4xl font-bold">"Welcome to Encampus"</h1>
          </header>
          <main class="my-8 text-center">
            <p class="mb-4 text-xl">"Making Learning Accessible and Engaging for Everyone."</p>
            <A
              href="/login"
              class="py-2 px-4 font-bold text-white bg-blue-500 rounded hover:bg-blue-700"
            >
              "Login to your account"
            </A>
          </main>
          <footer class="py-8 mt-auto">
            <p class="text-center">"© 2024 Encampus"</p>
          </footer>
        </div>
      </div>
    }
}
