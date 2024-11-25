/**
 * Highlighter icon component, used in the rich text editor
 */
use leptos::{component, view, IntoView};

#[component]
pub fn HighlighterIcon(
    /// css style size (E.g. sized="20px")
    /// (Note: Currently only supports static strings.
    ///  If you require dynamic sizing edit this component to take in a closure instead.)
    size: &'static str,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" width=size height=size viewBox="0 0 256 256">
        <path fill="currentColor" d="M200.12 55.87A102 102 0 1 0 55.88 200.12A102 102 0 1 0 200.12 55.87M94 211.37V152a2 2 0 0 1 2-2h64a2 2 0 0 1 2 2v59.37a90.5 90.5 0 0 1-68 0M146 138h-36V99.71l36-18Zm45.64 53.64A91 91 0 0 1 174 205.39V152a14 14 0 0 0-14-14h-2V72a6 6 0 0 0-8.68-5.37l-48 24A6 6 0 0 0 98 96v42h-2a14 14 0 0 0-14 14v53.39a91 91 0 0 1-17.64-13.75a90 90 0 1 1 127.28 0"/>
        </svg>
    }
}
