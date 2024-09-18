use crate::resources::images::svgs::drop_down_bars::*;
use leptos::{
    component, create_resource, server, view, IntoView, ServerFnError, SignalSet, Suspense,
};

#[component]
fn ClassCard() -> impl IntoView {
    let class_name = create_resource(
        || {},
        |_| async move { get_class_name().await.unwrap_or("Failed".to_string()) },
    );

    view! {
      <div class="flex flex-col justify-center items-center p-4 h-32 font-bold bg-white rounded shadow">
        <Suspense fallback=move || {
          view! { <p>"Loading..."</p> }
        }>{move || class_name().unwrap_or_default()}</Suspense>
        <button
          class="text-sm border"
          on:click=move |_| {
            class_name.set("Loading...".to_string());
            class_name.refetch();
          }
        >
          "Click Me To Send Request to Server (watch the network requests)"
        </button>
      </div>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
      <div class="flex justify-between items-center p-4 text-gray-600 bg-white">
        <LogoAndTitle />
        <div class="relative">
          <input
            type="text"
            placeholder="Ask me anything..."
            class="p-2 w-64 rounded-full border border-gray-300"
          />
          <button class="absolute top-0 right-0 mr-2 text-black">
            <i class="fa-search" />
          </button>
        </div>
        <div class="flex items-center">
          // TODO: Replace with vh/vw for dynamic size
          <span class="mr-4 text-xl font-bold">"LONGNAME"</span>
          <DropDownBars size="20px" />
        </div>
      </div>
    }
}

#[component]
fn LogoAndTitle() -> impl IntoView {
    view! {
      <div class="flex items-center">
        <img src="images/logo.png" alt="ENCAMPUS" class="mr-2 h-8" />
        <span class="text-xl font-bold">"ENCAMPUS"</span>
      </div>
    }
}

#[component]
pub fn TestPage() -> impl IntoView {
    view! {
      <div class="min-h-screen bg-gray-200">
        <NavBar />
        // TODO: Dynamically generate tiles
        <div class="grid grid-cols-3 gap-4 p-10 mx-20">
          <ClassCard />
          <ClassCard />
          <ClassCard />
          <ClassCard />
          <ClassCard />
        </div>
      </div>
    }
}

// Server Functions //

#[server(GetClassName)]
async fn get_class_name() -> Result<String, ServerFnError> {
    use leptos::{server_fn::error::NoCustomError, use_context};
    use sqlx::postgres::PgPool;

    // It's a little unsafe to just
    let pool = use_context::<PgPool>().ok_or(ServerFnError::<NoCustomError>::ServerError(
        "Unable to complete Request".to_string(),
    ))?;

    let row: (i32, String, String) = sqlx::query_as("SELECT * from classes LIMIT 1")
        .fetch_one(&pool)
        .await
        .expect("select should work");

    Ok(row.1)
}
