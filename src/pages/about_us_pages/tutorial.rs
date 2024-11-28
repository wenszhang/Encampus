use leptos::{component, view, IntoView};

#[component]
pub fn Tutorial() -> impl IntoView {
    view! {
      <iframe
        width="560"
        height="315"
        src="https://www.youtube.com/embed/ZAPuSBolj1U"
        title="YouTube video player"
        frameborder="0"
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
        allowfullscreen
      ></iframe>
    }
}
