use leptos::{component, view, IntoView};
use leptos_router::A;

/// Renders the home page of your application.
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <A href="/dev">"Dev Page"</A>
        <br/>
        <A href="/classes">"Classes Page"</A>
        <A href="/login">"Login Page"</A>
        <br/>
    }
}
