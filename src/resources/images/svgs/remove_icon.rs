/**
 * Dots icon component, used in card headers.
 */
use leptos::{component, view, IntoView};

#[component]
pub fn RemoveIcon(
    /// css style size (E.g. sized="20px")
    /// (Note: Currently only supports static strings.
    ///  If you require dynamic sizing edit this component to take in a closure instead.)
    size: &'static str,
) -> impl IntoView {
    view! {
      <svg xmlns="http://www.w3.org/2000/svg" width=size height=size viewBox="0 0 24 24">
        <g fill="currentColor">
          <path
            fill-rule="evenodd"
            d="M17 5V4a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2v1H4a1 1 0 0 0 0 2h1v11a3 3 0 0 0 3 3h8a3 3 0 0 0 3-3V7h1a1 1 0 1 0 0-2zm-2-1H9v1h6zm2 3H7v11a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1z"
            clip-rule="evenodd"
          />
          <path d="M9 9h2v8H9zm4 0h2v8h-2z" />
        </g>
      </svg>
    }
}
