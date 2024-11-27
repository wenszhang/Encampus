use leptos::{component, view, IntoView};

#[component]
pub fn AboutUs() -> impl IntoView {
    view! {
      <div class="flex flex-col justify-center items-center h-screen">
        <div class="container flex flex-col items-center px-4 mx-auto bg-white rounded-lg">
          <header class="flex justify-between items-center py-4">
            <h1 class="text-4xl font-bold">"About Us"</h1>
          </header>
          <main class="my-8 text-left">
            <h1 class="mb-4 text-xl">"Project Overview:"</h1>
            <p class="mb-4 text-xl">
              "Encampus is a modern classroom assistance platform designed to streamline communication and foster
              collaboration within academic environments. Built with Leptos, a framework powered by Rust for exceptional 
              performance and reliability, Encampus enables students to ask public and private questions beyond regular hours, 
              bridging the gap between traditional interactions and digital convenience. It acts as a dynamic forum where 
              students can share insights, answer each other's questions, and access AI-powered responses, all while benefiting 
              from real-time notifications and participation tracking. Designed to accommodate the busy schedules of diverse college 
              students, Encampus enhances in-person and remote interactions by refining the core functionalities of similar tools, 
              providing a seamless, efficient, and innovative solution for academic success."
            </p>
            <h1 class="mb-4 text-xl">"Team Members:"</h1>
            <div class="grid grid-cols-2 gap-4">
              <img src="/images/placeholder.png" alt="Gabe" class="w-32 h-32 rounded-full" />
              <div>
                <div class="border-b border-b-black">
                  <p class="text-xl">"Gabriel Famodu"</p>
                  <p>"Contact Info"</p>
                </div>
                <p>"Bio Here"</p>
              </div>
              <div>
                <div class="border-b border-b-black">
                  <p class="text-xl">"Jack Shunn"</p>
                  <p>"Contact Info"</p>
                </div>
                <p>
                  "I’m a computer science student with a passion for innovative technologies. My academic and professional journey has been
                  marked by a commitment to cutting-edge technological exploration and practical application.\n During my undergraduate studies, 
                  I assisted with research in Dr Jeff Phillips’ data science lab, developing a strong foundation in analytical approaches to 
                  software development. Professionally, I've gained valuable experience working on web applications at Lucid and Redo, where I 
                  honed my skills in creating robust and efficient software solutions.\n My technical experience centers on web development 
                  and emerging technologies including AI development. This passion resulted in my role as tech lead for our capstone project, 
                  where I made the non-standard decision to use Rust as our fullstack language paired with Leptos, an emerging web framework 
                  still in active development.\n\n My approach combines academic rigor with practical innovation, always seeking to push the 
                  boundaries of what's possible in software development. I'm particularly excited about exploring."
                </p>
              </div>
              <img src="/images/placeholder.png" alt="Jack" class="w-32 h-32 rounded-full" />
              <img src="/images/placeholder.png" alt="Matthew" class="w-32 h-32 rounded-full" />
              <div>
                <div class="border-b border-b-black">
                  <p class="text-xl">"Matthew Taylor"</p>
                  <p>"Contact Info"</p>
                </div>
                <p>
                  "My name is Matthew and I’m a senior Computer Science student at the University of Utah, preparing to graduate with a
                  strong foundation in both high-level and low-level programming languages. Over the past 3.5 years, I’ve worked as an intern 
                  QA Developer for the University’s UIT department, where I wrote and maintained an automated test suite using Cucumber and 
                  Java. This role honed my skills in software quality assurance, automation, and problem-solving while balancing my academic 
                  responsibilities.\n\n For our project, I ventured into Rust—a language I had no prior experience with. Diving into Rust 
                  for this project has showcased my adaptability and ability to quickly learn and apply new technologies. The project also 
                  reinforced my passion for writing high-performance code and exploring new tools that push the boundaries of efficient computing.\n\n
                  My interests lie in artificial intelligence, performance optimization, and solving complex problems through innovative software 
                  solutions. I thrive on challenges that require creative thinking and technical expertise, whether it’s designing efficient algorithms 
                  or debugging intricate systems.\n\n Beyond academics and work, I am driven by curiosity and enjoy expanding my skill set to stay 
                  ahead in the ever-evolving tech landscape. As I approach graduation, I’m excited to bring my technical knowledge, adaptability, 
                  and passion for problem-solving to new opportunities."
                </p>
              </div>
              <div>
                <div class="border-b border-b-black">
                  <p class="text-xl">"Wensen Zhang"</p>
                  <p>"Contact Info"</p>
                </div>
                <p>"Bio Here"</p>
              </div>
              <img src="/images/placeholder.png" alt="Wensen" class="w-32 h-32 rounded-full" />
              <img src="/images/placeholder.png" alt="Wentao" class="w-32 h-32 rounded-full" />
              <div>
                <div class="border-b border-b-black">
                  <p class="text-xl">"Wentao Zhang"</p>
                  <p>"Contact Info"</p>
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
