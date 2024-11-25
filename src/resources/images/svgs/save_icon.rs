/**
 * Save icon component, used within edit post on focused post view.
 */
use leptos::{component, view, IntoView};

#[component]
pub fn SaveIcon(
    /// css style size (E.g. sized="20px")
    /// (Note: Currently only supports static strings.
    ///  If you require dynamic sizing edit this component to take in a closure instead.)
    size: &'static str,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" width=size height=size viewBox="0 0 24 24">
        <path fill="white" d="M6 4h10.59L20 7.41V18a3 3 0 0 1-3 3H6a3 3 0 0 1-3-3V7a3 3 0 0 1 3-3m0 1a2 2 0 0 0-2 2v11a2 2 0 0 0 2 2h11a2 2 0 0 0 2-2V7.91L16.09 5H15v5H6zm1 0v4h7V5zm5 7a3 3 0 0 1 3 3a3 3 0 0 1-3 3a3 3 0 0 1-3-3a3 3 0 0 1 3-3m0 1a2 2 0 0 0-2 2a2 2 0 0 0 2 2a2 2 0 0 0 2-2a2 2 0 0 0-2-2"/>
        </svg>
    }
}
