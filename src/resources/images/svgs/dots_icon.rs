/**
 * Dots icon component, used in card headers.
 */
use leptos::{component, view, IntoView};

#[component]
pub fn DotsIcon(
    /// css style size (E.g. sized="20px")
    /// (Note: Currently only supports static strings.
    ///  If you require dynamic sizing edit this component to take in a closure instead.)
    size: &'static str,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" width="4em" height="4em" viewBox="0 0 256 256"><path fill="gray" d="M144 128a16 16 0 1 1-16-16a16 16 0 0 1 16 16m-84-16a16 16 0 1 0 16 16a16 16 0 0 0-16-16m136 0a16 16 0 1 0 16 16a16 16 0 0 0-16-16"/></svg>
    }
}
