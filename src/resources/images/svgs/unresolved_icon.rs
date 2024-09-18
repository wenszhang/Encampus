/**
 * Unresolved post component, used in unresolved post tag.
 */
use leptos::{component, view, IntoView};

#[component]
pub fn UnresolvedIcon(
    /// css style size (E.g. sized="20px")
    /// (Note: Currently only supports static strings.
    ///  If you require dynamic sizing edit this component to take in a closure instead.)
    size: &'static str,
) -> impl IntoView {
    view! {
      <svg xmlns="http://www.w3.org/2000/svg" width=size height=size viewBox="0 0 16 16">
        <g fill="currentColor">
          <path
            fill-rule="evenodd"
            d="M1.5 2h13l.5.5v6.854a4 4 0 0 0-1-.819V3H2v8h2.5l.5.5v1.793l2.146-2.147L7.5 11h.626q-.124.481-.126 1h-.293l-2.853 2.854L4 14.5V12H1.5l-.5-.5v-9z"
            clip-rule="evenodd"
          />
          <path d="M12 9a3 3 0 1 0 0 6a3 3 0 0 0 0-6" />
        </g>
      </svg>
    }
}
