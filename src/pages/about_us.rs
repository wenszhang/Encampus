use leptos::{component, view, IntoView};

#[component]
pub fn AboutUs() -> impl IntoView {
    view! {
      <div class="flex overflow-y-auto flex-col justify-center items-center min-h-screen bg-gray-100">
        <div class="container flex flex-col items-center py-6 px-6 mx-auto max-w-4xl bg-white rounded-lg">
          <header class="flex sticky top-0 z-10 justify-between items-center py-4 w-full bg-white">
            <h1 class="text-4xl font-bold">"About Us"</h1>
          </header>
          <main class="my-8 leading-relaxed text-left">
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
            <div class="grid grid-cols-1 gap-6 sm:grid-cols-1 md:grid-cols-2">
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
              <img
                src="/images/placeholder.png"
                alt="Matthew"
                class="object-cover overflow-hidden w-32 h-32 rounded-full"
              />
              <div>
                <div class="mb-4 border-b border-black">
                  <p class="text-xl">"Matthew Taylor"</p>
                  <p>"matthew.s.taylor21@gmail.com"</p>
                </div>
                <p>
                  "My name is Matthew and I’m a senior Computer Science student at the University of Utah, preparing to graduate with a
                  strong foundation in both high-level and low-level programming languages. Over the past 3.5 years, I’ve worked as an intern 
                  QA Developer for the University’s UIT department, where I wrote and maintained an automated test suite using Cucumber and 
                  Java. This role honed my skills in software quality assurance, automation, and problem-solving while balancing my academic 
                  responsibilities."
                </p>
                <p>
                  "For our project, I ventured into Rust—a language I had no prior experience with. Diving into Rust
                  for this project has showcased my adaptability and ability to quickly learn and apply new technologies. The project also 
                  reinforced my passion for writing high-performance code and exploring new tools that push the boundaries of efficient computing."
                </p>
                <p>
                  "My interests lie in artificial intelligence, performance optimization, and solving complex problems through innovative software
                  solutions. I thrive on challenges that require creative thinking and technical expertise, whether it’s designing efficient algorithms 
                  or debugging intricate systems."
                </p>
                <p>
                  "Beyond academics and work, I am driven by curiosity and enjoy expanding my skill set to stay
                  ahead in the ever-evolving tech landscape. As I approach graduation, I’m excited to bring my technical knowledge, adaptability, 
                  and passion for problem-solving to new opportunities."
                </p>
              </div>
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
                class="object-cover overflow-hidden w-32 h-32 rounded-full"
              />
              <img
                src="/images/placeholder.png"
                alt="Wentao"
                class="object-cover overflow-hidden w-32 h-32 rounded-full"
              />
              <div>
                <div class="mb-4 border-b border-black">
                  <p class="text-xl">"Wentao Zhang"</p>
                  <p>"wentao.zhg@utah.edu"</p>
                </div>
                <p>"Bio Here"</p>
              </div>
            </div>
          </main>
          <footer class="py-8 mt-auto">
            <p class="text-center">"© 2024 Encampus"</p>
          </footer>
        </div>
      </div>
    }
}
