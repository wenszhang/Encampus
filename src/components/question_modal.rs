use leptos::{component, view, IntoView};

#[component]
pub fn Modal() -> impl IntoView {
    view! {
        <button on:click=move |_| set_show_modal.set(true)>"Open Modal"</button>

        <Show when=move || show_modal.get() fallback=|| ()>
            <div node_ref=modal_ref class="modal">
                <div class="inner">
                    <button
                        class="button small"
                        title="Close"
                        on:click=move |_| set_show_modal.set(false)
                    >
                        "𝖷"
                    </button>
                    <p class="heading">"Test Modal"</p>
                    <p>"Body text"</p>
                </div>
            </div>
        </Show>
    }
}
