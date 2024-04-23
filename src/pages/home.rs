use crate::components::header::Header;
use leptos::{component, view, IntoView};
use leptos_router::A;

/// Renders the home page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Header text="ENCAMPUS".to_string() logo="logo.png".to_string() />
        <div class="container mx-auto px-4">
            <header class="flex justify-between items-center py-4">
                <h1 class="text-4xl font-bold">"Welcome to Encampus"</h1>
            </header>
            <main class="my-8">
                <p class="text-xl mb-4">"Supper snazzy tagline to draw your interest"</p>
                <A
                    href="/login"
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                >
                    "Login to your account"
                </A>
            </main>
            <footer class="py-4">
                <p>"© 2024 Encampus"</p>
            </footer>
        </div>
    }
}
