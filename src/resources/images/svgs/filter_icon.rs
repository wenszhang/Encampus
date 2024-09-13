/**
 * A Filter icon used within search bar
 */
use leptos::*;

/// A hamburger menu icon svg with custom size
#[component]
pub fn FilterIcon(
    /// css style size (E.g. sized="20px")
    /// (Note: Currently only supports static strings.
    ///  If you require dynamic sizing edit this component to take in a closure instead.)
    size: &'static str,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" width=size height=size viewBox="0 0 20 20"><path fill="white" d="M7.5 13h5a.5.5 0 0 1 .09.992L12.5 14h-5a.5.5 0 0 1-.09-.992zh5zm-2-4h9a.5.5 0 0 1 .09.992L14.5 10h-9a.5.5 0 0 1-.09-.992zh9zm-2-4h13a.5.5 0 0 1 .09.992L16.5 6h-13a.5.5 0 0 1-.09-.992zh13z"/><
        /svg>
    }
}
