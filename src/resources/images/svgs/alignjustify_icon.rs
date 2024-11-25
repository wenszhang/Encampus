/**
 * Align justify icon component, used in the rich text editor
 */
use leptos::{component, view, IntoView};

#[component]
pub fn AlignJustifyIcon(
    /// css style size (E.g. sized="20px")
    /// (Note: Currently only supports static strings.
    ///  If you require dynamic sizing edit this component to take in a closure instead.)
    size: &'static str,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" width=size height=size viewBox="0 0 24 24">
        <path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M3 6h18M3 10h18M3 14h18M3 18h18"/>
        </svg>
    }
}
