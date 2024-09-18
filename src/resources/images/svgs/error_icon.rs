/**
 * Error icon component, used for any error.
 */
use leptos::{component, view, IntoView};

#[component]
pub fn ErrorIcon(
    /// css style size (E.g. sized="20px")
    /// (Note: Currently only supports static strings.
    ///  If you require dynamic sizing edit this component to take in a closure instead.)
    size: &'static str,
) -> impl IntoView {
    view! {
      <svg xmlns="http://www.w3.org/2000/svg" width=size height=size viewBox="0 0 48 48">
        <path
          fill="red"
          fill-rule="evenodd"
          stroke="red"
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="4"
          d="m6 11l5-5l13 13L37 6l5 5l-13 13l13 13l-5 5l-13-13l-13 13l-5-5l13-13z"
          clip-rule="evenodd"
        />
      </svg>
    }
}
