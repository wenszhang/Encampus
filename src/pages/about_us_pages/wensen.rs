use leptos::{component, view, IntoView};

#[component]
pub fn Wensen() -> impl IntoView {
    view! {
      <div>
        <div class="mb-4 border-b border-black">
          <p class="text-xl">"Wensen Zhang"</p>
          <p>"wensen.zhang@utah.edu"</p>
        </div>
        <p>"Bio Here"</p>
      </div>
      <img
        src="/images/placeholder.png"
        alt="Wensen"
        class="object-cover overflow-hidden rounded-lg w-45 h-75"
      />
    }
}
