/**
 * down arrow component, used for denoting collapsable announcements
 */
use leptos::{component, view, IntoView};

#[component]
pub fn DownArrow(
    /// css style size (E.g. sized="20px")
    /// (Note: Currently only supports static strings.
    ///  If you require dynamic sizing edit this component to take in a closure instead.)
    size: &'static str,
) -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" width="2em" height="2em" viewBox="0 0 40 40"><path fill="white" d="M4.659 11.833h30.682L20 32.167z"/></svg>
    }
}
