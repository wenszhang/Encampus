/**
 * Pin icon component, used as an action on a question tile.
 */
use leptos::{component, view, IntoView};

#[component]
pub fn PinIcon(
    /// css style size (E.g. sized="20px")
    /// (Note: Currently only supports static strings.
    ///  If you require dynamic sizing edit this component to take in a closure instead.)
    size: &'static str,
) -> impl IntoView {
    view! {
      <svg xmlns="http://www.w3.org/2000/svg" width=size height=size viewBox="0 0 2048 2048">
        <path
          fill="currentColor"
          d="M1990 748q-33 33-64 60t-66 47t-73 29t-89 11q-34 0-65-6l-379 379q13 38 19 78t6 80q0 65-13 118t-37 100t-60 89t-79 87l-386-386l-568 569l-136 45l45-136l569-568l-386-386q44-44 86-79t89-59t100-38t119-13q40 0 80 6t78 19l379-379q-6-31-6-65q0-49 10-88t30-74t46-65t61-65z"
        />
      </svg>
    }
}
