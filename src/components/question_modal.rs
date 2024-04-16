use leptos::{ev::MouseEvent, *};

#[component]
pub fn QuestionModal<F>(on_close: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! {
        // Cover the viewport with an overlay and use TailwindCSS for styling
        <div class="fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center"
             on:click=on_close>
            // Disable main view click events and use TailwindCSS for the modal
            <div class="bg-white p-6 rounded-lg shadow-xl"
                 on:click=move |event| {
                     event.stop_propagation();
                 }>
                "This is a blank modal."
            </div>
        </div>
    }
}
