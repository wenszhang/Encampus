/**
 * Encampus Assistant icon component, used an AI reply. When the assistant replies to a post,.
 */
use leptos::{component, view, IntoView};

#[component]
pub fn EncampusAssistantIcon(
    /// css style size (E.g. sized="20px")
    /// (Note: Currently only supports static strings.
    ///  If you require dynamic sizing edit this component to take in a closure instead.)
    size: &'static str,
) -> impl IntoView {
    view! {
      <svg xmlns="http://www.w3.org/2000/svg" width=size height=size viewBox="0 0 48 48">
        <rect width="48" height="48" fill="none" />
        <defs>
          <linearGradient id="blueToPurple" gradientTransform="rotate(90)">
            <stop offset="0%" stop-color="#60a5fa" />
            <stop offset="100%" stop-color="#9333ea" />
          </linearGradient>
        </defs>
        <path
          fill="url(#blueToPurple)"
          d="M34 6c-1.368 4.944-3.13 6.633-8 8c4.87 1.367 6.632 3.056 8 8c1.368-4.944 3.13-6.633 8-8c-4.87-1.367-6.632-3.056-8-8m-14 8c-2.395 8.651-5.476 11.608-14 14c8.524 2.392 11.605 5.349 14 14c2.395-8.651 5.476-11.608 14-14c-8.524-2.392-11.605-5.349-14-14"
        />
      </svg>
    }
}
