/**
 * A hamburger menu icon svg with custom size
 */
use leptos::*;

/// A hamburger menu icon svg with custom size
#[component]
pub fn EndorsedIcon(
    /// css style size (E.g. sized="20px")
    /// (Note: Currently only supports static strings.
    ///  If you require dynamic sizing edit this component to take in a closure instead.)
    size: &'static str,
) -> impl IntoView {
    view! {
      <svg xmlns="http://www.w3.org/2000/svg" width=size height=size viewBox="0 0 24 24">
        <path
          fill="currentColor"
          d="m5.825 21l1.625-7.025L2 9.25l7.2-.625L12 2l2.8 6.625l7.2.625l-5.45 4.725L18.175 21L12 17.275z"
        />
      </svg>
    }
}
