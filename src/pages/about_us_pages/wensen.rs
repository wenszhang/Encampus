use leptos::{component, view, IntoView};

#[component]
pub fn Wensen() -> impl IntoView {
    view! {
      <div>
        <div class="mb-4 border-b border-black">
          <p class="text-xl">"Wensen Zhang"</p>
          <p>"wensen.zhang@utah.edu"</p>
        </div>
        <p>
          "My name is Wensen, and I am pursuing a Bachelor of Science in Computer Science with a focus on full-stack web development. I began my career at Pikfarm, where I contributed to the development of a geo-location-based online marketplace connecting local farmers with consumers using React Native and Kotlin. After high school, I spent four years at University Support Services as a Web Engineer Intern. During this time, I helped create and maintain faculty and student support tools, including systems such as View/Enter Grades, Academic Reports, and Course Feedback."
        </p>

        <p>
          "My academic background includes 32 credits in UI/UX design, 16 credits in full-stack web application development, and 12 credits in digital content creation, providing me with a versatile skill set for both development and design-oriented roles."
        </p>

        <p>
          "As part of my capstone project, I collaborated with a team to develop a student help forum using Rust and Leptos. My primary role on the Encampus project was bridging the gap between design and implementation, addressing roadblocks by rapidly delivering critical features to ensure the project's success."
        </p>

        <p>
          "I am passionate about creating scalable and maintainable software, with a focus on modularity to reduce technical debt and enhance long-term usability. Additionally, I am motivated by opportunities to innovate and develop solutions that directly reflect user experiences and improve system efficiency."
        </p>
      </div>
      <img
        src="/images/about_us_headshots/Wensen.png"
        alt="Wensen"
        class="object-cover overflow-hidden rounded-lg w-45 h-75"
      />
    }
}
