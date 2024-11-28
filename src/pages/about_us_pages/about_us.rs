use leptos::{component, create_signal, view, IntoView, SignalGet};

use crate::pages::about_us_pages::{
    gabe::Gabe, jack::Jack, matt::Matt, team::Team, tutorial::Tutorial, wensen::Wensen,
    wentao::Wentao,
};

#[component]
pub fn AboutUs() -> impl IntoView {
    let (active_tab, set_active_tab) = create_signal("Team".to_string());

    view! {
      <div class="flex overflow-y-auto flex-col justify-center items-center min-h-screen bg-gray-100">
        <div class="container flex flex-col items-center py-6 px-6 mx-auto max-w-4xl bg-white rounded-lg">
          <header class="flex sticky top-0 z-10 justify-between items-center py-4 w-full bg-white">
            <h1 class="text-4xl font-bold">"About Us"</h1>
          </header>
          <main class="my-8 w-full leading-relaxed text-left">
            <div class="flex mb-6 space-x-4">
              <button
                class=move || {
                  format!("tab {}", if active_tab.get() == "$1" { "active" } else { "" })
                }
                on:click=move |_| set_active_tab("Team".to_string())
              >
                "Team"
              </button>
              <button
                class=move || {
                  format!("tab {}", if active_tab.get().as_str() == "Gabe" { "active" } else { "" })
                }
                on:click=move |_| set_active_tab("Gabe".to_string())
              >
                "Gabe"
              </button>
              <button
                class=move || {
                  format!("tab {}", if *active_tab.get() == "Jack" { "active" } else { "" })
                }
                on:click=move |_| set_active_tab("Jack".to_string())
              >
                "Jack"
              </button>
              <button
                class=move || {
                  format!("tab {}", if *active_tab.get() == "Matt" { "active" } else { "" })
                }
                on:click=move |_| set_active_tab("Matt".to_string())
              >
                "Matt"
              </button>
              <button
                class=move || {
                  format!("tab {}", if *active_tab.get() == "Wensen" { "active" } else { "" })
                }
                on:click=move |_| set_active_tab("Wensen".to_string())
              >
                "Wensen"
              </button>
              <button
                class=move || {
                  format!("tab {}", if *active_tab.get() == "Wentao" { "active" } else { "" })
                }
                on:click=move |_| set_active_tab("Wentao".to_string())
              >
                "Wentao"
              </button>
              <button
                class=move || {
                  format!("tab {}", if *active_tab.get() == "Tutorial" { "active" } else { "" })
                }
                on:click=move |_| set_active_tab("Tutorial".to_string())
              >
                "Tutorial"
              </button>
            </div>
            <div class="tab-content">
              {move || match active_tab.get().as_str() {
                "Team" => Team().into_view(),
                "Gabe" => Gabe().into_view(),
                "Jack" => Jack().into_view(),
                "Matt" => Matt().into_view(),
                "Wensen" => Wensen().into_view(),
                "Wentao" => Wentao().into_view(),
                "Tutorial" => Tutorial().into_view(),
                _ => view! { "" }.into_view(),
              }}
            </div>
          </main>
          <footer class="py-8 mt-auto">
            <p class="text-center">"© 2024 Encampus"</p>
          </footer>
        </div>
      </div>
    }
}
