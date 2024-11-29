/**
 * Save icon component, used within edit post on focused post view.
 */
use leptos::{component, view, IntoView};

#[component]
pub fn UnapproveIcon(
    /// css style size (E.g. sized="20px")
    /// (Note: Currently only supports static strings.
    ///  If you require dynamic sizing edit this component to take in a closure instead.)
    size: &'static str,
) -> impl IntoView {
    view! {
            <svg xmlns="http://www.w3.org/2000/svg" width=size height=size viewBox="0 0 20 20">
            <path fill="#B47E29" d="M10 1c-5 0-9 4-9 9s4 9 9 9s9-4 9-9s-4-9-9-9m0 16c-3.9 0-7-3.1-7-7s3.1-7 7-7s7 3.1 7 7s-3.1 7-7 7M6 9v2h8V9z" class="st0"/>
        </svg>
    }
}
