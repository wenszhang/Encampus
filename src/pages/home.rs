use crate::components::header::Header;
use leptos::{component, view, IntoView};
use leptos_router::A;

/// Renders the home page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        // <Header text="ENCAMPUS".to_string() logo="logo.png".to_string() />
        <div class="flex flex-col justify-center items-center h-screen">
            <div class="container mx-auto px-4 flex flex-col items-center">
                <header class="flex justify-between items-center py-4">
                    <h1 class="text-4xl font-bold">"Welcome to Encampus"</h1>
                </header>
                <main class="my-8 text-center">
                    <p class="text-xl mb-4">"Making Learning Accessible and Engaging for Everyone."</p>
                    <A
                        href="/login"
                        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
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
