use leptos::{component, view, IntoView};

#[component]
pub fn Jack() -> impl IntoView {
    view! {
      <div>
        <div class="mb-4 border-b border-black">
          <p class="text-xl">"Jack Shunn"</p>
          <p>"jack.shunn@gmail.com"</p>
        </div>
        <div class="space-y-2">
          <p>
            "I’m a computer science student with a passion for innovative technologies. My academic and professional journey has been
            marked by a commitment to cutting-edge technological exploration and practical application."
          </p>
          <p>
            "During my undergraduate studies,
            I assisted with research in Dr Jeff Phillips’ data science lab, developing a strong foundation in analytical approaches to 
            software development. Professionally, I've gained valuable experience working on web applications at Lucid and Redo, where I 
            honed my skills in creating robust and efficient software solutions."
          </p>
          <p>
            "My technical experience centers on web development
            and emerging technologies including AI development. This passion resulted in my role as tech lead for our capstone project, 
            where I made the non-standard decision to use Rust as our fullstack language paired with Leptos, an emerging web framework 
            still in active development."
          </p>
          <p>
            "My approach combines academic rigor with practical innovation, always seeking to push the
            boundaries of what's possible in software development. I'm particularly excited about exploring."
          </p>
        </div>
      </div>
      <img
        src="/images/about_us_headshots/Jack.jpg"
        alt="Jack"
        class="object-cover overflow-hidden rounded-lg w-45 h-75"
      />
    }
}
