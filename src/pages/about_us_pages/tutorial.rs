use leptos::{component, view, IntoView};

#[component]
pub fn Tutorial() -> impl IntoView {
    view! {
      <div class="flex justify-center items-center min-h-screen bg-gray-100">
        <div class="p-6 bg-white rounded-lg shadow-lg">
          <iframe
            class="rounded-md"
            width="560"
            height="315"
            src="https://www.youtube.com/embed/ZAPuSBolj1U"
            title="YouTube video player"
            frameborder="0"
            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
            allowfullscreen
          ></iframe>
        </div>
      </div>
    }
}
