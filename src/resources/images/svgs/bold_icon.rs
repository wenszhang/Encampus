/**
 * Bold icon component, used in the rich text editor
 */
use leptos::{component, view, IntoView};

#[component]
pub fn BoldIcon(
    /// css style size (E.g. sized="20px")
    /// (Note: Currently only supports static strings.
    ///  If you require dynamic sizing edit this component to take in a closure instead.)
    size: &'static str,
) -> impl IntoView {
    view! {
        <svg class="mr-2 w-5 h-5" width=size height=size xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16">
        <path d="M4 2h4.5a3.501 3.501 0 0 1 2.852 5.53A3.499 3.499 0 0 1 9.5 14H4a1 1 0 0 1-1-1V3a1 1 0 0 1 1-1m1 7v3h4.5a1.5 1.5 0 0 0 0-3Zm3.5-2a1.5 1.5 0 0 0 0-3H5v3Z" />
      </svg>
    }
}
