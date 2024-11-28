use leptos::{component, view, IntoView};

#[component]
pub fn Wentao() -> impl IntoView {
    view! {
      <img
        src="/images/about_us_headshots/Wentao.png"
        alt="Wentao"
        class="object-cover overflow-hidden rounded-lg w-45 h-75"
      />
      <div>
        <div class="mb-4 border-b border-black">
          <p class="text-xl">"Wentao Zhang"</p>
          <p>"wentao.zhg@utah.edu"</p>
        </div>
        <div>
          <p>
            "Hi, I'm Wentao, I am a Computer Science major studying at the University of Utah expecting to graduate this semester. I have been
            interested in computer science ever since I was a kid, enjoying aspects such as artificial intelligence and software engineering. 
            Over the past 3 years, I have worked as a Full Stack Software Engineer Intern here at University of Utah helping maintain and design 
            features for large scale Angular web applications. This experience helped me gain valuable insight on professional development techniques 
            and ethics."
          </p>

          <p>
            "Throughout my time as a Student, I have also been heavily involved in AI and Machine learning research and development with an
            emphasis in visualization and mathematical modeling. I have taken plenty of coursework designed to facilitate creation and training of 
            machine learning models as well as data processing and visualization."
          </p>

          <p>
            "For the Senior Capstone Project, our team created a web application built on Rust using the Leptos Framework to help improve student
            and instructor communication, targeting question resolution. Here I took a full stack engineer role working on creation of both front and
            backend elements to create a cohesive user experience whether it's asking or answering a question among peers."
          </p>
        </div>
      </div>
    }
}
