/**
 * Peper icon component, used in Custom post tag.
 */
use leptos::{component, view, IntoView};

#[component]
pub fn PaperIcon(
    /// css style size (E.g. sized="20px")
    /// (Note: Currently only supports static strings.
    ///  If you require dynamic sizing edit this component to take in a closure instead.)
    size: &'static str,
) -> impl IntoView {
    view! {
      <svg xmlns="http://www.w3.org/2000/svg" width=size height=size viewBox="0 0 24 24">
        <path
          fill="currentColor"
          d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8zm4 18H6V4h7v5h5z"
        />
      </svg>
    }
}
