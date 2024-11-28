use leptos::{component, view, IntoView};

#[component]
pub fn Matt() -> impl IntoView {
    view! {
      <img
        src="/images/about_us_headshots/Matthew.jpg"
        alt="Matthew"
        class="object-cover overflow-hidden rounded-lg w-45 h-75"
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
    }
}
