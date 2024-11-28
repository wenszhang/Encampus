use leptos::{component, view, IntoView};

#[component]
pub fn Team() -> impl IntoView {
    view! {
      <h1 class="mb-4 text-xl font-bold">"Project Overview:"</h1>
      <p class="mb-6 text-lg">
        "Encampus is a modern classroom assistance platform designed to streamline communication and foster
        collaboration within academic environments. Built with Leptos, a framework powered by Rust for exceptional 
        performance and reliability, Encampus enables students to ask public and private questions beyond regular hours, 
        bridging the gap between traditional interactions and digital convenience. It acts as a dynamic forum where 
        students can share insights, answer each other's questions, and access AI-powered responses, all while benefiting 
        from real-time notifications and participation tracking. Designed to accommodate the busy schedules of diverse college 
        students, Encampus enhances in-person and remote interactions by refining the core functionalities of similar tools, 
        providing a seamless, efficient, and innovative solution for academic success."
      </p>
      <h1 class="mb-4 text-xl font-bold">"Team Members:"</h1>
    }
}
