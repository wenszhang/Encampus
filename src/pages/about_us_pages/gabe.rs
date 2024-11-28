use leptos::{component, view, IntoView};

#[component]
pub fn Gabe() -> impl IntoView {
    view! {
      <img
        src="/images/about_us_headshots/Gabe.png"
        alt="Gabe"
        class="object-cover overflow-hidden rounded-lg w-45 h-75"
      />
      <div>
        <div class="mb-4 border-b border-black">
          <p class="text-xl">"Gabriel Famodu"</p>
          <p>"gabefamodu13@gmail.com"</p>
        </div>
        <div class="space-y-2">
          <p>
            "Hello, I'm Gabriel! I am a Computer Science major here at the University of Utah. Prior to starting my degree, I gained valuable experience in the startup world through founding Wed Rentals, where I saw firsthand how technology could make a real difference in people's lives. This experience, along with my deep passion for creating practical solutions through technology that positively impact people's daily lives, led me to where I am today."
          </p>
          <p>
            "For my capstone project, I served as a full stack engineer focusing on developing solutions to improve online academic assistance. My startup background helped me approach our project with a user-centered mindset and assisted me with conducting research through interviews with students, teaching assistants, and professors at the university to ensure our platform effectively meets the needs of our academic community."
          </p>
          <p>
            "Always eager to challenge myself, I embraced the opportunity to work with Rust for our project despite having no prior experience with the language. The commitment to learning new technologies and pushing my boundaries exemplifies my approach to software development. My interests lie in artificial intelligence and full stack web applications. As my college life comes to a close, I continue to seek opportunities to leverage technology and make a meaningful impact on the world."
          </p>
        </div>
      </div>
    }
}
